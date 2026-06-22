import { describe, expect, it } from "vitest";
import {
  addToastMessage,
  createToastMessage,
  dismissToastMessage,
  MAX_VISIBLE_TOASTS,
} from "./toastState";

describe("toastState", () => {
  it("creates an info toast by default", () => {
    expect(createToastMessage("toast-1", { title: "Saved" })).toEqual({
      id: "toast-1",
      title: "Saved",
      description: undefined,
      variant: "info",
    });
  });

  it("adds newest toast first and keeps the visible list bounded", () => {
    const messages = Array.from({ length: MAX_VISIBLE_TOASTS }, (_, index) =>
      createToastMessage(`toast-${index}`, { title: `Toast ${index}` }),
    );
    const nextMessage = createToastMessage("toast-new", { title: "Newest" });

    const nextMessages = addToastMessage(messages, nextMessage);

    expect(nextMessages).toHaveLength(MAX_VISIBLE_TOASTS);
    expect(nextMessages[0]).toBe(nextMessage);
    expect(nextMessages.at(-1)?.id).toBe("toast-2");
  });

  it("dismisses a toast by id", () => {
    const messages = [
      createToastMessage("toast-1", { title: "First" }),
      createToastMessage("toast-2", { title: "Second" }),
    ];

    expect(dismissToastMessage(messages, "toast-1")).toEqual([messages[1]]);
  });
});
