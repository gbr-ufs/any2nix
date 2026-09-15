/*
 * SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

import { CodeJar } from "/node_modules/codejar/dist/codejar.js";

function highlight(element) {
    if (window.Prism) {
        window.Prism.highlightElement(element);
    }
}

export function initEditor(element) {
    if (element._codejar) {
        return element._codejar;
    }

    const jar = CodeJar(element, highlight);

    const form = element.closest("form");
    const format = element.id ? element.id.replace("editor-", "") : "";
    const hiddenInput = form ? (form.querySelector(`input[type="hidden"]#${format}`) || form.querySelector(`input[type="hidden"][name="input"]`)) : null;

    jar.onUpdate((code) => {
        if (hiddenInput) {
            hiddenInput.value = code;
        }
    });

    if (hiddenInput && hiddenInput.value) {
        jar.updateCode(hiddenInput.value);
    } else if (element.textContent.trim()) {
        if (hiddenInput) {
            hiddenInput.value = element.textContent;
        }
    }

    element._codejar = jar;

    if (form) {
        const fileInput = form.querySelector('input[type="file"].file-input');
        if (fileInput && !fileInput._fileInit) {
            fileInput._fileInit = true;
            fileInput.addEventListener("change", async () => {
                const file = fileInput.files?.[0];
                if (file) {
                    try {
                        const text = await file.text();
                        jar.updateCode(text);
                        if (hiddenInput) {
                            hiddenInput.value = text;
                        }
                    } catch (err) {
                        console.error("Failed to read file:", err);
                    }
                }
                fileInput.value = "";
            });
        }
    }

    if (!element._dragInit) {
        element._dragInit = true;

        element.addEventListener("dragover", (e) => {
            e.preventDefault();
            e.stopPropagation();
            element.classList.add("dragover");
        });

        element.addEventListener("dragleave", (e) => {
            e.preventDefault();
            e.stopPropagation();
            element.classList.remove("dragover");
        });

        element.addEventListener("drop", async (e) => {
            e.preventDefault();
            e.stopPropagation();
            element.classList.remove("dragover");

            const file = e.dataTransfer?.files?.[0];
            if (file) {
                try {
                    const text = await file.text();
                    jar.updateCode(text);
                    if (hiddenInput) {
                        hiddenInput.value = text;
                    }
                } catch (err) {
                    console.error("Failed to read dropped file:", err);
                }
            }
        });
    }

    return jar;
}

export function initAllEditors(root = document) {
    root.querySelectorAll(".editor").forEach(initEditor);
}

export function initDownloadButtons(root = document) {
    root.querySelectorAll(".download-nix-button").forEach((button) => {
        if (button._downloadInit) {
            return;
        }
        button._downloadInit = true;

        button.addEventListener("click", () => {
            const container = button.closest(".page") || button.closest("main") || document;
            const codeEl = container.querySelector("code.language-nix");
            if (!codeEl) {
                return;
            }

            const blob = new Blob([codeEl.textContent], { type: "text/plain;charset=utf-8" });
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = "default.nix";
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a);
            URL.revokeObjectURL(url);
        });
    });
}

export function enhanceCopyButtons(root = document) {
    root.querySelectorAll(".copy-to-clipboard-button").forEach((button) => {
        if (!button.querySelector("i")) {
            const icon = document.createElement("i");
            icon.textContent = "content_copy";
            icon.setAttribute("aria-hidden", "true");
            button.prepend(icon);

            const observer = new MutationObserver(() => {
                const state = button.getAttribute("data-copy-state");
                if (state === "copy-success") {
                    icon.textContent = "check";
                } else {
                    icon.textContent = "content_copy";
                }
            });
            observer.observe(button, { attributes: true, attributeFilter: ["data-copy-state"] });
        }
    });
}

function refreshUI() {
    initAllEditors();
    initDownloadButtons();
    if (window.Prism) {
        Prism.highlightAll();
        enhanceCopyButtons();
    }
}

document.addEventListener("DOMContentLoaded", () => {
    refreshUI();
});

document.addEventListener("htmx:afterSwap", () => {
    refreshUI();
});

document.addEventListener("htmx:load", () => {
    refreshUI();
});
