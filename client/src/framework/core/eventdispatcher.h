/**
 * Shadow OT Client - Event Dispatcher
 *
 * Central event system for decoupled component communication.
 */

#pragma once

#include <string>
#include <functional>
#include <map>
#include <vector>
#include <memory>
#include <queue>
#include <mutex>
#include <any>

namespace shadow {
namespace framework {

struct Event {
    std::string type;
    std::map<std::string, std::any> data;

    template<typename T>
    T get(const std::string& key) const {
        auto it = data.find(key);
        if (it != data.end()) {
            return std::any_cast<T>(it->second);
        }
        return T{};
    }

    template<typename T>
    void set(const std::string& key, T value) {
        data[key] = std::move(value);
    }
};

class EventDispatcher {
public:
    static EventDispatcher& instance();

    using EventCallback = std::function<void(const Event&)>;
    using CallbackId = uint64_t;

    // Subscribe to events
    CallbackId subscribe(const std::string& eventType, EventCallback callback);
    void unsubscribe(CallbackId id);
    void unsubscribeAll(const std::string& eventType);

    // Dispatch events
    void dispatch(const Event& event);
    void dispatchImmediate(const Event& event);
    void dispatchDelayed(const Event& event, double delayMs);

    // Process queued events
    void poll();

    // Event creation helpers
    static Event createEvent(const std::string& type) {
        Event e;
        e.type = type;
        return e;
    }

    // Scheduled events
    void scheduleEvent(const std::string& type, double intervalMs, bool repeat = false);
    void cancelScheduled(const std::string& type);

private:
    EventDispatcher() = default;
    ~EventDispatcher() = default;
    EventDispatcher(const EventDispatcher&) = delete;
    EventDispatcher& operator=(const EventDispatcher&) = delete;

    struct ScheduledEvent {
        Event event;
        double triggerTime;
        double interval;
        bool repeat;
    };

    struct Subscription {
        CallbackId id;
        std::string eventType;
        EventCallback callback;
    };

    std::mutex m_mutex;
    std::vector<Subscription> m_subscriptions;
    std::queue<Event> m_eventQueue;
    std::vector<ScheduledEvent> m_scheduledEvents;
    CallbackId m_nextCallbackId{1};
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::EventDispatcher& g_dispatcher;
