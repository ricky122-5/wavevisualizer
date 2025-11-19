#!/bin/bash

IDLE_THRESHOLD=300
MULTI_OUTPUT_DEVICE="Multi-Output Device"
PLAYLIST_NAME="Aux"
PROJECT_DIR="/Users/rickyreddygari/Documents/rustproj/musicdash"

TEST_MODE=false
if [ "$1" = "--test" ] || [ "$1" = "-t" ]; then
    TEST_MODE=true
fi

get_idle_time() {
    idle_time=$(ioreg -c IOHIDSystem | awk '/HIDIdleTime/ {print int($NF/1000000000); exit}')
    
    if [ -z "$idle_time" ] || [ "$idle_time" = "0" ]; then
        idle_time=$(python3 -c "
import subprocess
result = subprocess.run(['ioreg', '-c', 'IOHIDSystem'], capture_output=True, text=True)
for line in result.stdout.split('\n'):
    if 'HIDIdleTime' in line:
        ns = int(line.split('=')[1].strip())
        print(int(ns / 1000000000))
        break
" 2>/dev/null)
    fi
    
    echo "${idle_time:-0}"
}

set_audio_device() {
    osascript <<EOF
tell application "System Events"
    set volume output volume 50
end tell

tell application "Audio MIDI Setup"
    -- This requires the SwitchAudioSource tool
end tell
EOF
    
    if command -v SwitchAudioSource &> /dev/null; then
        SwitchAudioSource -s "$MULTI_OUTPUT_DEVICE"
        echo "✓ Switched to $MULTI_OUTPUT_DEVICE"
    else
        echo "⚠ SwitchAudioSource not found. Install with: brew install switchaudio-osx"
    fi
}

play_music() {
    osascript <<EOF
tell application "Music"
    if not running then
        launch
        delay 2
    end if
    
    -- Enable shuffle
    set shuffle enabled to true
    set shuffle mode to songs
    
    try
        play playlist "$PLAYLIST_NAME"
        set sound volume to 50
    on error
        -- If playlist not found, just play any music with shuffle
        set shuffle enabled to true
        play
    end try
end tell
EOF
    echo "✓ Started playing music (shuffle enabled)"
}

main() {
    echo "🎵 Music Visualizer Auto-Launcher"
    echo "================================"
    
    if [ "$TEST_MODE" = true ]; then
        echo "⚠️  TEST MODE - Skipping idle check"
    else
        idle=$(get_idle_time)
        echo "Current idle time: ${idle}s (threshold: ${IDLE_THRESHOLD}s)"
        
        if [ "$idle" -lt "$IDLE_THRESHOLD" ]; then
            echo "❌ System not idle enough. Exiting."
            exit 0
        fi
        
        echo "✓ System has been idle for ${idle}s"
    fi
    
    echo "Setting audio device..."
    set_audio_device
    
    echo "Starting music..."
    play_music
    
    sleep 2
    
    echo "Launching visualizer..."
    cd "$PROJECT_DIR" || exit 1
    cargo run
}

main
