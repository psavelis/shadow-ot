/**
 * Shadow OT Client - Event Dispatcher Implementation
 */

#include "eventdispatcher.h"
#include <framework/core/application.h>
#include <algorithm>

namespace shadow {
namespace framework {

EventDispatcher& EventDispatcher::instance() {
    static EventDispatcher instance;
    return instance;
}

EventDispatcher::CallbackId EventDispatcher::subscribe(const std::string& eventType, EventCallback callback) {
    std::lock_guard<std::mutex> lock(m_mutex);

    Subscription sub;
    sub.id = m_nextCallbackId++;
    sub.eventType = eventType;
    sub.callback = std::move(callback);

    m_subscriptions.push_back(std::move(sub));

    return sub.id;
}

void EventDispatcher::unsubscribe(CallbackId id) {
    std::lock_guard<std::mutex> lock(m_mutex);

    m_subscriptions.erase(
        std::remove_if(m_subscriptions.begin(), m_subscriptions.end(),
            [id](const Subscription& sub) { return sub.id == id; }),
        m_subscriptions.end()
    );
}

void EventDispatcher::unsubscribeAll(const std::string& eventType) {
    std::lock_guard<std::mutex> lock(m_mutex);

    m_subscriptions.erase(
        std::remove_if(m_subscriptions.begin(), m_subscriptions.end(),
            [&eventType](const Subscription& sub) { return sub.eventType == eventType; }),
        m_subscriptions.end()
    );
}

void EventDispatcher::dispatch(const Event& event) {
    std::lock_guard<std::mutex> lock(m_mutex);
    m_eventQueue.push(event);
}

void EventDispatcher::dispatchImmediate(const Event& event) {
    std::vector<EventCallback> callbacks;

    {
        std::lock_guard<std::mutex> lock(m_mutex);
        for (const auto& sub : m_subscriptions) {
            if (sub.eventType == event.type) {
                callbacks.push_back(sub.callback);
            }
        }
    }

    for (const auto& callback : callbacks) {
        callback(event);
    }
}

void EventDispatcher::dispatchDelayed(const Event& event, double delayMs) {
    std::lock_guard<std::mutex> lock(m_mutex);

    ScheduledEvent scheduled;
    scheduled.event = event;
    scheduled.triggerTime = g_app.getFrameTime() + (delayMs / 1000.0);
    scheduled.interval = 0;
    scheduled.repeat = false;

    m_scheduledEvents.push_back(std::move(scheduled));
}

void EventDispatcher::scheduleEvent(const std::string& type, double intervalMs, bool repeat) {
    std::lock_guard<std::mutex> lock(m_mutex);

    ScheduledEvent scheduled;
    scheduled.event = createEvent(type);
    scheduled.triggerTime = g_app.getFrameTime() + (intervalMs / 1000.0);
    scheduled.interval = intervalMs / 1000.0;
    scheduled.repeat = repeat;

    m_scheduledEvents.push_back(std::move(scheduled));
}

void EventDispatcher::cancelScheduled(const std::string& type) {
    std::lock_guard<std::mutex> lock(m_mutex);

    m_scheduledEvents.erase(
        std::remove_if(m_scheduledEvents.begin(), m_scheduledEvents.end(),
            [&type](const ScheduledEvent& se) { return se.event.type == type; }),
        m_scheduledEvents.end()
    );
}

void EventDispatcher::poll() {
    double currentTime = g_app.getFrameTime();

    // Process scheduled events
    {
        std::lock_guard<std::mutex> lock(m_mutex);

        for (auto it = m_scheduledEvents.begin(); it != m_scheduledEvents.end();) {
            if (currentTime >= it->triggerTime) {
                m_eventQueue.push(it->event);

                if (it->repeat) {
                    it->triggerTime = currentTime + it->interval;
                    ++it;
                } else {
                    it = m_scheduledEvents.erase(it);
                }
            } else {
                ++it;
            }
        }
    }

    // Process queued events
    while (true) {
        Event event;
        {
            std::lock_guard<std::mutex> lock(m_mutex);
            if (m_eventQueue.empty()) break;
            event = m_eventQueue.front();
            m_eventQueue.pop();
        }

        dispatchImmediate(event);
    }
}

} // namespace framework
} // namespace shadow

// Global instance
shadow::framework::EventDispatcher& g_dispatcher = shadow::framework::EventDispatcher::instance();
