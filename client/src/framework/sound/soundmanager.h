/**
 * Shadow OT Client - Sound Manager
 *
 * Audio system with support for sound effects, music, and 3D positional audio.
 */

#pragma once

#include <string>
#include <map>
#include <vector>
#include <memory>
#include <functional>
#include <cstdint>

namespace shadow {
namespace framework {

// Sound channel types
enum class SoundChannel : int {
    Effects = 0,
    Music = 1,
    Ambient = 2,
    UI = 3,
    Voice = 4,
    MaxChannels = 8
};

// Sound file formats supported
enum class SoundFormat {
    Unknown,
    WAV,
    OGG,
    MP3,
    FLAC
};

// Sound effect instance
class SoundEffect {
public:
    SoundEffect() = default;
    ~SoundEffect() = default;

    bool load(const std::string& filename);
    void unload();

    bool isLoaded() const { return m_loaded; }
    const std::string& getFilename() const { return m_filename; }
    float getDuration() const { return m_duration; }

private:
    std::string m_filename;
    bool m_loaded{false};
    float m_duration{0.0f};

    // Audio data (would be backend-specific)
    void* m_audioData{nullptr};
    size_t m_audioSize{0};
};

using SoundEffectPtr = std::shared_ptr<SoundEffect>;

// Music track
class MusicTrack {
public:
    MusicTrack() = default;
    ~MusicTrack() = default;

    bool load(const std::string& filename);
    void unload();

    bool isLoaded() const { return m_loaded; }
    const std::string& getFilename() const { return m_filename; }
    const std::string& getTitle() const { return m_title; }
    float getDuration() const { return m_duration; }

private:
    std::string m_filename;
    std::string m_title;
    bool m_loaded{false};
    float m_duration{0.0f};

    // Streaming handle
    void* m_streamHandle{nullptr};
};

using MusicTrackPtr = std::shared_ptr<MusicTrack>;

// Playing sound handle
struct PlayingSoundHandle {
    uint32_t id{0};
    bool valid{false};
};

// Sound source for 3D audio
struct SoundSource3D {
    float x{0.0f};
    float y{0.0f};
    float z{0.0f};
    float volume{1.0f};
    float minDistance{1.0f};
    float maxDistance{100.0f};
};

class SoundManager {
public:
    static SoundManager& instance();

    bool init();
    void terminate();
    void update(float deltaTime);

    // Sound effects
    SoundEffectPtr loadSound(const std::string& filename);
    void unloadSound(const std::string& filename);
    PlayingSoundHandle playSound(const std::string& filename, SoundChannel channel = SoundChannel::Effects);
    PlayingSoundHandle playSound(SoundEffectPtr sound, SoundChannel channel = SoundChannel::Effects);
    PlayingSoundHandle playSound3D(const std::string& filename, const SoundSource3D& source);

    // Music
    MusicTrackPtr loadMusic(const std::string& filename);
    void playMusic(const std::string& filename, bool loop = true, float fadeIn = 0.0f);
    void playMusic(MusicTrackPtr track, bool loop = true, float fadeIn = 0.0f);
    void stopMusic(float fadeOut = 0.0f);
    void pauseMusic();
    void resumeMusic();
    bool isMusicPlaying() const { return m_musicPlaying; }
    bool isMusicPaused() const { return m_musicPaused; }
    const std::string& getCurrentMusicFilename() const;

    // Sound control
    void stopSound(PlayingSoundHandle handle);
    void stopAllSounds(SoundChannel channel);
    void stopAllSounds();
    bool isPlaying(PlayingSoundHandle handle) const;

    // Volume control (0.0 to 1.0)
    void setMasterVolume(float volume);
    float getMasterVolume() const { return m_masterVolume; }

    void setChannelVolume(SoundChannel channel, float volume);
    float getChannelVolume(SoundChannel channel) const;

    void setMusicVolume(float volume);
    float getMusicVolume() const { return m_musicVolume; }

    void setSoundVolume(float volume);
    float getSoundVolume() const { return m_soundVolume; }

    // Muting
    void setMuted(bool muted);
    bool isMuted() const { return m_muted; }

    void setMusicMuted(bool muted);
    bool isMusicMuted() const { return m_musicMuted; }

    void setSoundMuted(bool muted);
    bool isSoundMuted() const { return m_soundMuted; }

    // Listener position for 3D audio
    void setListenerPosition(float x, float y, float z);
    void setListenerOrientation(float atX, float atY, float atZ, float upX, float upY, float upZ);

    // Preloading
    void preloadSounds(const std::vector<std::string>& filenames);
    void clearCache();

    // Callbacks
    using MusicFinishedCallback = std::function<void()>;
    void setMusicFinishedCallback(MusicFinishedCallback callback) { m_musicFinishedCallback = callback; }

    // Audio device info
    std::string getAudioDeviceName() const { return m_deviceName; }
    bool isInitialized() const { return m_initialized; }

private:
    SoundManager() = default;

    float getEffectiveVolume(SoundChannel channel) const;

    // State
    bool m_initialized{false};
    std::string m_deviceName;

    // Volume settings
    float m_masterVolume{1.0f};
    float m_musicVolume{0.7f};
    float m_soundVolume{1.0f};
    std::map<SoundChannel, float> m_channelVolumes;

    // Mute settings
    bool m_muted{false};
    bool m_musicMuted{false};
    bool m_soundMuted{false};

    // Music state
    bool m_musicPlaying{false};
    bool m_musicPaused{false};
    bool m_musicLooping{false};
    MusicTrackPtr m_currentMusic;
    float m_musicFadeTime{0.0f};
    float m_musicFadeTarget{0.0f};
    float m_musicFadeCurrent{1.0f};

    // Sound cache
    std::map<std::string, SoundEffectPtr> m_soundCache;
    std::map<std::string, MusicTrackPtr> m_musicCache;

    // Playing sounds tracking
    uint32_t m_nextSoundId{1};
    std::map<uint32_t, SoundChannel> m_playingSounds;

    // 3D listener
    float m_listenerX{0.0f};
    float m_listenerY{0.0f};
    float m_listenerZ{0.0f};

    // Callbacks
    MusicFinishedCallback m_musicFinishedCallback;
};

} // namespace framework
} // namespace shadow

// Global accessor
extern shadow::framework::SoundManager& g_sounds;
