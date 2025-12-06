/**
 * Shadow OT Client - Sound Manager Implementation
 */

#include "soundmanager.h"
#include <algorithm>
#include <cmath>

namespace shadow {
namespace framework {

// SoundEffect implementation

bool SoundEffect::load(const std::string& filename) {
    m_filename = filename;

    // In a full implementation, this would:
    // 1. Load the audio file (WAV, OGG, etc.)
    // 2. Decode to PCM
    // 3. Create audio buffer (SDL_mixer chunk, OpenAL buffer, etc.)

    // Determine format from extension
    SoundFormat format = SoundFormat::Unknown;
    if (filename.ends_with(".wav")) format = SoundFormat::WAV;
    else if (filename.ends_with(".ogg")) format = SoundFormat::OGG;
    else if (filename.ends_with(".mp3")) format = SoundFormat::MP3;
    else if (filename.ends_with(".flac")) format = SoundFormat::FLAC;

    m_loaded = true;
    return m_loaded;
}

void SoundEffect::unload() {
    if (m_audioData) {
        // Free audio buffer
        m_audioData = nullptr;
    }
    m_loaded = false;
}

// MusicTrack implementation

bool MusicTrack::load(const std::string& filename) {
    m_filename = filename;

    // In a full implementation, this would:
    // 1. Open the audio file for streaming
    // 2. Read metadata (title, duration, etc.)
    // 3. Create streaming handle

    m_loaded = true;
    return m_loaded;
}

void MusicTrack::unload() {
    if (m_streamHandle) {
        // Close stream
        m_streamHandle = nullptr;
    }
    m_loaded = false;
}

// SoundManager implementation

SoundManager& SoundManager::instance() {
    static SoundManager instance;
    return instance;
}

bool SoundManager::init() {
    if (m_initialized) return true;

    // Initialize audio subsystem
    // In a full implementation, this would:
    // 1. Initialize SDL_mixer or OpenAL
    // 2. Open audio device
    // 3. Set up channels

    // Set default channel volumes
    for (int i = 0; i < static_cast<int>(SoundChannel::MaxChannels); ++i) {
        m_channelVolumes[static_cast<SoundChannel>(i)] = 1.0f;
    }

    m_initialized = true;
    m_deviceName = "Default Audio Device";

    return true;
}

void SoundManager::terminate() {
    if (!m_initialized) return;

    stopAllSounds();
    stopMusic();

    clearCache();

    // Close audio subsystem
    m_initialized = false;
}

void SoundManager::update(float deltaTime) {
    if (!m_initialized) return;

    // Handle music fading
    if (m_musicFadeTime > 0.0f) {
        float fadeProgress = deltaTime / m_musicFadeTime;
        m_musicFadeTime -= deltaTime;

        if (m_musicFadeTime <= 0.0f) {
            m_musicFadeCurrent = m_musicFadeTarget;
            m_musicFadeTime = 0.0f;

            // If fading to 0, stop music
            if (m_musicFadeTarget <= 0.0f) {
                m_musicPlaying = false;
            }
        } else {
            float diff = m_musicFadeTarget - m_musicFadeCurrent;
            m_musicFadeCurrent += diff * fadeProgress;
        }

        // Apply volume change
        // setMusicVolumeInternal(m_musicFadeCurrent * m_musicVolume * m_masterVolume);
    }

    // Clean up finished sounds
    // In a full implementation, check which sounds have finished playing
}

SoundEffectPtr SoundManager::loadSound(const std::string& filename) {
    // Check cache first
    auto it = m_soundCache.find(filename);
    if (it != m_soundCache.end()) {
        return it->second;
    }

    // Load new sound
    auto sound = std::make_shared<SoundEffect>();
    if (sound->load(filename)) {
        m_soundCache[filename] = sound;
        return sound;
    }

    return nullptr;
}

void SoundManager::unloadSound(const std::string& filename) {
    auto it = m_soundCache.find(filename);
    if (it != m_soundCache.end()) {
        it->second->unload();
        m_soundCache.erase(it);
    }
}

PlayingSoundHandle SoundManager::playSound(const std::string& filename, SoundChannel channel) {
    auto sound = loadSound(filename);
    if (!sound) {
        return {0, false};
    }
    return playSound(sound, channel);
}

PlayingSoundHandle SoundManager::playSound(SoundEffectPtr sound, SoundChannel channel) {
    if (!m_initialized || !sound || !sound->isLoaded()) {
        return {0, false};
    }

    if (m_muted || m_soundMuted) {
        return {0, false};
    }

    // Calculate effective volume
    float volume = getEffectiveVolume(channel);

    // In a full implementation:
    // 1. Find a free channel
    // 2. Set volume
    // 3. Play sound

    uint32_t soundId = m_nextSoundId++;
    m_playingSounds[soundId] = channel;

    return {soundId, true};
}

PlayingSoundHandle SoundManager::playSound3D(const std::string& filename, const SoundSource3D& source) {
    if (!m_initialized || m_muted || m_soundMuted) {
        return {0, false};
    }

    auto sound = loadSound(filename);
    if (!sound) {
        return {0, false};
    }

    // Calculate distance-based volume
    float dx = source.x - m_listenerX;
    float dy = source.y - m_listenerY;
    float dz = source.z - m_listenerZ;
    float distance = std::sqrt(dx * dx + dy * dy + dz * dz);

    float attenuation = 1.0f;
    if (distance > source.minDistance) {
        if (distance >= source.maxDistance) {
            attenuation = 0.0f;
        } else {
            attenuation = 1.0f - (distance - source.minDistance) /
                                 (source.maxDistance - source.minDistance);
        }
    }

    float volume = source.volume * attenuation * getEffectiveVolume(SoundChannel::Effects);

    if (volume <= 0.0f) {
        return {0, false};
    }

    // In a full implementation, also calculate pan based on direction

    uint32_t soundId = m_nextSoundId++;
    m_playingSounds[soundId] = SoundChannel::Effects;

    return {soundId, true};
}

MusicTrackPtr SoundManager::loadMusic(const std::string& filename) {
    auto it = m_musicCache.find(filename);
    if (it != m_musicCache.end()) {
        return it->second;
    }

    auto music = std::make_shared<MusicTrack>();
    if (music->load(filename)) {
        m_musicCache[filename] = music;
        return music;
    }

    return nullptr;
}

void SoundManager::playMusic(const std::string& filename, bool loop, float fadeIn) {
    auto music = loadMusic(filename);
    if (music) {
        playMusic(music, loop, fadeIn);
    }
}

void SoundManager::playMusic(MusicTrackPtr track, bool loop, float fadeIn) {
    if (!m_initialized || !track || !track->isLoaded()) return;

    // Stop current music if playing
    if (m_musicPlaying) {
        stopMusic(0.0f);
    }

    m_currentMusic = track;
    m_musicLooping = loop;
    m_musicPlaying = true;
    m_musicPaused = false;

    if (fadeIn > 0.0f) {
        m_musicFadeCurrent = 0.0f;
        m_musicFadeTarget = 1.0f;
        m_musicFadeTime = fadeIn;
    } else {
        m_musicFadeCurrent = 1.0f;
        m_musicFadeTarget = 1.0f;
        m_musicFadeTime = 0.0f;
    }

    // In a full implementation, start playing the music stream
}

void SoundManager::stopMusic(float fadeOut) {
    if (!m_musicPlaying) return;

    if (fadeOut > 0.0f) {
        m_musicFadeTarget = 0.0f;
        m_musicFadeTime = fadeOut;
    } else {
        m_musicPlaying = false;
        m_musicPaused = false;
        m_currentMusic = nullptr;

        // Actually stop playback
    }
}

void SoundManager::pauseMusic() {
    if (m_musicPlaying && !m_musicPaused) {
        m_musicPaused = true;
        // Pause playback
    }
}

void SoundManager::resumeMusic() {
    if (m_musicPlaying && m_musicPaused) {
        m_musicPaused = false;
        // Resume playback
    }
}

const std::string& SoundManager::getCurrentMusicFilename() const {
    static std::string empty;
    return m_currentMusic ? m_currentMusic->getFilename() : empty;
}

void SoundManager::stopSound(PlayingSoundHandle handle) {
    if (!handle.valid) return;

    auto it = m_playingSounds.find(handle.id);
    if (it != m_playingSounds.end()) {
        // Stop the sound
        m_playingSounds.erase(it);
    }
}

void SoundManager::stopAllSounds(SoundChannel channel) {
    for (auto it = m_playingSounds.begin(); it != m_playingSounds.end();) {
        if (it->second == channel) {
            // Stop sound
            it = m_playingSounds.erase(it);
        } else {
            ++it;
        }
    }
}

void SoundManager::stopAllSounds() {
    m_playingSounds.clear();
    // Stop all sound channels
}

bool SoundManager::isPlaying(PlayingSoundHandle handle) const {
    if (!handle.valid) return false;
    return m_playingSounds.find(handle.id) != m_playingSounds.end();
}

void SoundManager::setMasterVolume(float volume) {
    m_masterVolume = std::clamp(volume, 0.0f, 1.0f);
}

void SoundManager::setChannelVolume(SoundChannel channel, float volume) {
    m_channelVolumes[channel] = std::clamp(volume, 0.0f, 1.0f);
}

float SoundManager::getChannelVolume(SoundChannel channel) const {
    auto it = m_channelVolumes.find(channel);
    return it != m_channelVolumes.end() ? it->second : 1.0f;
}

void SoundManager::setMusicVolume(float volume) {
    m_musicVolume = std::clamp(volume, 0.0f, 1.0f);
}

void SoundManager::setSoundVolume(float volume) {
    m_soundVolume = std::clamp(volume, 0.0f, 1.0f);
}

void SoundManager::setMuted(bool muted) {
    m_muted = muted;
}

void SoundManager::setMusicMuted(bool muted) {
    m_musicMuted = muted;
}

void SoundManager::setSoundMuted(bool muted) {
    m_soundMuted = muted;
}

void SoundManager::setListenerPosition(float x, float y, float z) {
    m_listenerX = x;
    m_listenerY = y;
    m_listenerZ = z;
}

void SoundManager::setListenerOrientation(float atX, float atY, float atZ,
                                          float upX, float upY, float upZ) {
    // Set listener orientation for 3D audio
}

void SoundManager::preloadSounds(const std::vector<std::string>& filenames) {
    for (const auto& filename : filenames) {
        loadSound(filename);
    }
}

void SoundManager::clearCache() {
    for (auto& [filename, sound] : m_soundCache) {
        sound->unload();
    }
    m_soundCache.clear();

    for (auto& [filename, music] : m_musicCache) {
        music->unload();
    }
    m_musicCache.clear();
}

float SoundManager::getEffectiveVolume(SoundChannel channel) const {
    if (m_muted || m_soundMuted) return 0.0f;
    return m_masterVolume * m_soundVolume * getChannelVolume(channel);
}

} // namespace framework
} // namespace shadow

// Global accessor
shadow::framework::SoundManager& g_sounds = shadow::framework::SoundManager::instance();
