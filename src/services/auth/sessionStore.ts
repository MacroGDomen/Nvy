import type { AccountSession } from "../desktopApi/types";

let currentSession: AccountSession | null = null;

export function setSession(session: AccountSession) {
  currentSession = session;
}

export function getSession() {
  return currentSession;
}

export function clearSession() {
  currentSession = null;
}

export function hasSession() {
  return currentSession !== null;
}

export function requireSession() {
  if (!currentSession) {
    throw new Error("Current account is required.");
  }

  return currentSession;
}
