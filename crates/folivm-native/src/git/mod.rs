// git2-rs abstraction for project versioning.
//
// Responsibilities:
// - Auto-save commits (background, debounced)
// - Named version commits ("Save Version" command)
// - Draft branch management
// - Diff generation for the version history diff view
//
// The git layer operates on serialised `.fvm` strings. It never parses
// document content — that is folivm-core's job.
