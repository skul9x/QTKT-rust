# Phase 01: UI Bypass - Direct Landing
Status: ✅ Complete
Dependencies: None

## Objective
Remove the restriction that requires an API key to access the main application interface.

## Requirements
### Functional
- [x] Modify `src/routes/+page.svelte` to remove the `needsOnboarding` check that blocks the main UI.
- [x] Ensure the main layout (Sidebar + Main content) renders immediately after a quick loading check.
- [x] (Optional) Clean up unused `Onboarding` component imports if deemed unnecessary.

## Implementation Steps
1. [x] Edit `src/routes/+page.svelte`: Remove `{#if needsOnboarding}` conditional block.
2. [x] Simplify `checkApiKey` logic: It should still fetch the key into state (for status display) but not set a "blocker" flag.
3. [x] Verify that the application boots directly into the Generator screen.

## Files to Create/Modify
- `src/routes/+page.svelte` - Remove onboarding conditional logic.

## Test Criteria
- [ ] App starts and shows Sidebar/Generator without asking for a key.
- [ ] Navigation between tabs works correctly on first boot.
