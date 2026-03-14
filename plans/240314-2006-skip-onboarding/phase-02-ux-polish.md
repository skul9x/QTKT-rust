# Phase 02: UX Polish - Missing Key Warning
Status: ✅ Complete
Dependencies: Phase 01

## Objective
Improve the user experience when a user tries to generate a document without having an API key configured.

## Requirements
### Functional
- [x] Add a warning/info message in `src/lib/components/screens/Generator.svelte` if the topic is entered but the API key is missing.
- [x] Provide a direct link or button to the "Settings" tab when the key is missing.
- [x] Gracefully handle the error in the logs if `generate_qtkt` is called without a valid key.

## Implementation Steps
1. [x] Edit `src/lib/components/screens/Generator.svelte`: Add a reactive check for API key presence.
2. [x] Add a conditional UI element (alert or banner) prompting to "Go to Settings" if no key is found.
3. [x] Update `handleGenerate` to check for key presence before calling the backend.

## Files to Create/Modify
- `src/lib/components/screens/Generator.svelte` - Add UI warnings and pre-flight checks.

## Test Criteria
- [ ] If no key exists, a warning is visible on the Generator page.
- [ ] Clicking the "Go to Settings" link switches the active tab.
- [ ] Proper error message appears in the logs if generation fails due to missing key.
