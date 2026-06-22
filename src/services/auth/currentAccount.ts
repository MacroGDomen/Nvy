import type { AccountSession } from "../desktopApi/types";
import {
  clearSession,
  getSession,
  requireSession,
  setSession,
} from "./sessionStore";

export function setCurrentAccount(session: AccountSession) {
  setSession(session);
}

export function getCurrentAccount() {
  return getSession();
}

export function clearCurrentAccount() {
  clearSession();
}

export function requireCurrentAccount() {
  return requireSession();
}
