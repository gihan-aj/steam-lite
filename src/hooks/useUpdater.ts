import { getVersion } from "@tauri-apps/api/app";
import { ask } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { useEffect, useState } from "react";

interface UpdateStatus {
  currentVersion: string | null;
  available: boolean;
  version: string | null;
  downloading: boolean;
  progress: number;
}

export function useUpdater() {
    const [state, setState] = useState<UpdateStatus>({
      currentVersion: null,
      available: false,
      version: null,
      downloading: false,
      progress: 0,
    });

    useEffect(() => {
        getVersion().then(v => {
            setState(s => ({...s, currentVersion: v}));
        });
    },[]);

    const checkForUpdates = async (silent = false) => {
        try {
          const update: Update | null = await check();

          if (!update) {
            if (!silent) console.log("[Updater] Already up to date");
            return;
          }

          console.log(`[Updater] Update available: ${update.version}`);
          setState((s) => ({
            ...s,
            availbale: true,
            version: update.version,
          }));

          // Ask user if they want to update
          const yes = await ask(
            `Version ${update.version} is available. \n\nWould you like to update now?`,
            {
              title: "Update Available",
              kind: "info",
              okLabel: "Update",
              cancelLabel: "Later",
            },
          );

          if (!yes) return;

          setState((s) => ({ ...s, downloading: true, progress: 0 }));

          let downloaded = 0;
          let totalSize = 0;

          await update.downloadAndInstall((event) => {
            switch (event.event) {
              case "Started":
                totalSize = event.data.contentLength ?? 0;
                setState((s) => ({ ...s, progress: 0 }));
                break;

              case "Progress":
                downloaded += event.data.chunkLength;
                if (totalSize > 0) {
                  const pct = Math.round((downloaded / totalSize) * 100);
                  setState((s) => ({ ...s, progress: Math.min(pct, 99) }));
                } else {
                  // No content length — show indeterminate progress
                  // Increment slowly to show something is happening
                  setState((s) => ({
                    ...s,
                    progress: Math.min(s.progress + 2, 90),
                  }));
                }
                break;

              case "Finished":
                setState((s) => ({ ...s, progress: 100 }));
                break;
            }
          });

          await relaunch();
        } catch (e) {
          // Silently fail — update check is not critical
          // This fires when GitHub releases URL returns 404
          // (before you've published any release)
          console.log(
            "[Updater] Update check failed (expected before first release):",
            e,
          );
          setState((s) => ({ ...s, downloading: false }));
        }
    };

    useEffect(() => {
        const timer = setTimeout(checkForUpdates, 5000);
        return () => clearTimeout(timer);
    }, []);

    return {...state, checkForUpdates: () => checkForUpdates(false) };
}