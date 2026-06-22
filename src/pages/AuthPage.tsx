import { FormEvent, useState } from "react";
import { Button } from "../components/ui/Button";
import { Input } from "../components/ui/Input";
import { loginAccount, registerAccount } from "../services/desktopApi";
import type { AccountSession } from "../services/desktopApi/types";
import { setCurrentAccount } from "../services/auth/currentAccount";

type AuthMode = "login" | "register";

type AuthPageProps = {
  onAuthenticated: (session: AccountSession) => void;
};

export function AuthPage({ onAuthenticated }: AuthPageProps) {
  const [mode, setMode] = useState<AuthMode>("login");
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    setError("");
    setIsSubmitting(true);

    try {
      const session =
        mode === "login"
          ? await loginAccount(username, password)
          : await registerAccount(username, password);

      setCurrentAccount(session);
      onAuthenticated(session);
    } catch (caughtError) {
      setError(toAuthMessage(caughtError));
    } finally {
      setIsSubmitting(false);
    }
  }

  function switchMode(nextMode: AuthMode) {
    setMode(nextMode);
    setError("");
  }

  const isLogin = mode === "login";

  return (
    <main className="min-h-screen px-5 py-6 text-[var(--color-text)]">
      <section className="mx-auto grid min-h-[calc(100vh-3rem)] max-w-6xl grid-cols-1 overflow-hidden rounded-[32px] border border-[var(--color-border)] bg-[rgba(22,21,29,0.72)] shadow-[var(--shadow-panel)] lg:grid-cols-[1.08fr_0.92fr]">
        <div className="relative flex min-h-[340px] flex-col justify-between overflow-hidden p-8 sm:p-10">
          <div className="absolute inset-0 bg-[radial-gradient(circle_at_24%_20%,rgba(159,136,219,0.28),transparent_24rem),radial-gradient(circle_at_78%_74%,rgba(211,134,170,0.2),transparent_22rem)]" />
          <div className="relative z-10">
            <div className="inline-grid h-14 min-w-24 place-items-center rounded-[20px] bg-[linear-gradient(135deg,var(--color-accent),var(--color-accent-pink))] px-5 text-2xl font-extrabold tracking-normal text-[var(--color-accent-foreground)]">
              NVY
            </div>
          </div>
          <div className="relative z-10 grid max-w-xl gap-5">
            <p className="m-0 text-sm font-semibold text-[var(--color-accent-soft)]">
              本地离线模式
            </p>
            <h1 className="m-0 text-5xl font-semibold leading-tight tracking-normal text-[var(--color-text-strong)] sm:text-6xl">
              进入你的本地资料库
            </h1>
            <p className="m-0 max-w-lg text-base leading-7 text-[var(--color-muted)]">
              第一版只使用 Windows 本地账号，数据保存在本机并按账号隔离。
            </p>
          </div>
        </div>

        <div className="grid content-center gap-7 border-t border-[var(--color-border)] bg-[rgba(18,17,24,0.76)] p-6 sm:p-10 lg:border-l lg:border-t-0">
          <div className="inline-grid grid-cols-2 rounded-full bg-[var(--color-surface-soft)] p-1">
            <button
              className={modeButtonClassName(isLogin)}
              type="button"
              onClick={() => switchMode("login")}
            >
              登录
            </button>
            <button
              className={modeButtonClassName(!isLogin)}
              type="button"
              onClick={() => switchMode("register")}
            >
              注册
            </button>
          </div>

          <form className="grid gap-5" onSubmit={handleSubmit}>
            <Input
              autoComplete="username"
              label="用户名"
              maxLength={10}
              minLength={3}
              name="username"
              placeholder="3-10 位中文、字母或数字"
              value={username}
              onChange={(event) => setUsername(event.target.value)}
            />
            <Input
              autoComplete={isLogin ? "current-password" : "new-password"}
              label="密码"
              maxLength={20}
              minLength={3}
              name="password"
              placeholder="3-20 位字母或数字"
              type="password"
              value={password}
              onChange={(event) => setPassword(event.target.value)}
            />

            {error ? (
              <p className="m-0 rounded-2xl border border-[rgba(183,91,116,0.34)] bg-[rgba(183,91,116,0.12)] px-4 py-3 text-sm leading-6 text-[var(--color-danger-foreground)]">
                {error}
              </p>
            ) : null}

            <Button className="h-12 w-full" disabled={isSubmitting} type="submit">
              {isSubmitting ? "处理中" : isLogin ? "登录本地账号" : "创建本地账号"}
            </Button>
          </form>

          <div className="grid gap-3 rounded-[24px] border border-[var(--color-border)] bg-[var(--color-surface-soft)] p-4 text-sm leading-6 text-[var(--color-muted)]">
            <div className="flex items-center justify-between gap-4">
              <span>运行模式</span>
              <span className="rounded-full bg-[rgba(159,136,219,0.14)] px-3 py-1 text-[var(--color-accent-soft)]">
                离线本地
              </span>
            </div>
            <div className="flex items-center justify-between gap-4">
              <span>默认推荐参考数</span>
              <span>30</span>
            </div>
          </div>
        </div>
      </section>
    </main>
  );
}

function modeButtonClassName(isActive: boolean) {
  return [
    "h-10 rounded-full px-4 text-sm font-medium tracking-normal transition",
    isActive
      ? "bg-[var(--color-accent)] text-[var(--color-accent-foreground)] shadow-[var(--shadow-glow)]"
      : "text-[var(--color-muted)] hover:bg-[var(--color-surface-hover)] hover:text-[var(--color-text)]",
  ].join(" ");
}

function toAuthMessage(error: unknown) {
  const message = String(error);

  if (message.includes("Username already exists")) {
    return "用户名已存在，请更换用户名或直接登录。";
  }

  if (message.includes("Account or password is incorrect")) {
    return "账号或密码不正确。";
  }

  if (message.includes("Username must be 3-10")) {
    return "用户名需要 3-10 位。";
  }

  if (message.includes("Username can only contain")) {
    return "用户名只能包含中文、字母和数字。";
  }

  if (message.includes("Password must be 3-20")) {
    return "密码需要 3-20 位。";
  }

  if (message.includes("Password can only contain")) {
    return "密码只能包含字母和数字。";
  }

  return "操作失败，请检查输入后重试。";
}
