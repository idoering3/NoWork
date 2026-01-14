import { load } from "@tauri-apps/plugin-store";

function createTimer() {
    // --- Private state & config ---
    let config = {
        studyTime: 25 * 60 * 1000,
        breakTime: 5 * 60 * 1000,
        repetitions: 4,
    };
    let timerInterval: ReturnType<typeof setInterval> | null = null;

    // --- Reactive state ---
    const audio = new Audio('win95boot.wav');
    let timeLimit = $state(config.studyTime);
    let timeLeft = $state(config.studyTime);
    let isStudying = $state(true);
    let sessionNum = $state(1);
    let isRunning = $state(false);
    let isEnabled = $state(false); // Controls if the timer should be active
    let timerFinished = $state(false);

    // --- Methods ---
    function pause() {
        if (timerInterval) {
            clearInterval(timerInterval);
            timerInterval = null;
            isRunning = false;
        }
    }

    async function start() {
        if (isRunning) return;
        isRunning = true;
        isEnabled = true;

        if (timerFinished) {
            timeLeft = timeLimit;
            sessionNum = 1;
            timerFinished = false;
        }

        let studyMinuteCounter = 0;

        timerInterval = setInterval(async () => {
            if (timeLeft > 100) {
                timeLeft -= 100;

                if (isStudying) {
                    studyMinuteCounter += 100;

                    if (studyMinuteCounter >= 60_000) {
                        studyMinuteCounter = 0;

                        const store = await load(".settings.json");
                        let value = await store.get<{ value: number }>("totalStudyTime");
                        let minutesStudied;
                        if (value?.value) {
                            minutesStudied = value.value;
                        } else {
                            minutesStudied = 0;
                        }
                        await store.set('totalStudyTime', {value: minutesStudied + 1});
                        await store.save();
                        console.log(minutesStudied);
                    }
                }
            } else {
                audio.play();
                pause(); // Pause to switch states
                
                if (isStudying && sessionNum >= config.repetitions) {
                    // Final study session finished, don’t start a break
                    audio.play();
                    timerFinished = true;
                    pause();
                    return;
                }
                
                if (isStudying) { // Switch to break
                    isStudying = false;
                    timeLimit = config.breakTime;
                    timeLeft = config.breakTime;
                } else { // Switch to study
                    isStudying = true;
                    sessionNum += 1;
                    timeLimit = config.studyTime;
                    timeLeft = config.studyTime;
                }
                // Short delay to allow state to update before restarting
                setTimeout(start, 10);
            }
        }, 100);
    }

    function reset() {
        pause();
        isStudying = true;
        sessionNum = 1;
        timeLimit = config.studyTime;
        timeLeft = config.studyTime;
        isEnabled = false; // This will signal the component to unmount
    }

    function init(newConfig: { studyTime: number; breakTime: number; repetitions: number; timerEnabled: boolean }) {
        config = { ...config, ...newConfig };
        isEnabled = newConfig.timerEnabled ?? isEnabled;
        timeLimit = config.studyTime;
        timeLeft = config.studyTime;
    }
    
    // --- Public API ---
    return {
        // Read-only state accessors
        get timeLeft() { return timeLeft; },
        get timeLimit() { return timeLimit; },
        get isStudying() { return isStudying; },
        get sessionNum() { return sessionNum; },
        get isRunning() { return isRunning; },
        get isEnabled() { return isEnabled; },
        get totalSessions() { return config.repetitions; },

        // Actions
        start,
        pause,
        reset,
        init,
    };
}

// Export a singleton instance of the timer
export const timerStore = createTimer();

// Export the utility function
export function msToMinutesSeconds(ms: number): string {
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`;
}