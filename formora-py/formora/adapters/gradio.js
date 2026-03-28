/**
 * Gradio Formora Adapter
 *
 * This adapter integrates Formora forms with Gradio chat interfaces.
 * It targets Gradio's chatbot textarea and triggers the submit action.
 *
 * INSTALLATION:
 * Add this script to your Gradio app's custom JavaScript block.
 *
 * USAGE:
 * The adapter automatically detects form submissions and injects the data.
 */

window.addEventListener('formora:submit', (e) => {
  // Gradio typically uses a textarea within a specific container
  const textarea = document.querySelector('textarea[class*="gradio"]') ||
                   document.querySelector('#component-0 textarea') ||
                   document.querySelector('.gradio-container textarea');

  if (textarea) {
    // Inject the formora message
    textarea.value = e.detail.raw;
    textarea.dispatchEvent(new Event('input', { bubbles: true }));

    // Find the submit button (Gradio uses various button patterns)
    const submitButton = document.querySelector('button[class*="submit"]') ||
                        document.querySelector('button.primary') ||
                        document.querySelector('button:contains("Submit")') ||
                        document.querySelector('#component-0 button');

    if (submitButton) {
      submitButton.click();
    }
  }
});

/**
 * TESTED VERSION: Gradio 4.x (latest as of 2025)
 */
