// Mutual Exclusion Model
// Models a simple mutex lock system

sig Process {}

sig State {
    waiting: set Process,
    critical: set Process,
    outside: set Process
}

// Facts that define valid states
fact ValidState {
    all s: State {
        // Every process is in exactly one region
        s.waiting + s.critical + s.outside = Process
        no s.waiting & s.critical
        no s.waiting & s.outside
        no s.critical & s.outside

        // At most one process in critical section (mutex property)
        lone s.critical
    }
}

// State transition: process enters waiting
pred enterWaiting[s, s': State, p: Process] {
    p in s.outside
    s'.waiting = s.waiting + p
    s'.critical = s.critical
    s'.outside = s.outside - p
}

// State transition: process enters critical section
pred enterCritical[s, s': State, p: Process] {
    p in s.waiting
    no s.critical  // Critical section must be empty
    s'.critical = s.critical + p
    s'.waiting = s.waiting - p
    s'.outside = s.outside
}

// State transition: process exits critical section
pred exitCritical[s, s': State, p: Process] {
    p in s.critical
    s'.outside = s.outside + p
    s'.critical = s.critical - p
    s'.waiting = s.waiting
}

// Assert: mutual exclusion is always maintained
assert MutualExclusion {
    all s: State | lone s.critical
}

// Assert: if a process is waiting, it can eventually enter
assert NoStarvation {
    all s: State, p: Process |
        p in s.waiting implies
            some s': State | p in s'.critical
}

check MutualExclusion for 5
run {} for 5 Process, 4 State
