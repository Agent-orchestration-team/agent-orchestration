import { randomUUID } from 'node:crypto';
import { mkdir, open, rename, rm } from 'node:fs/promises';
import { setTimeout as delay } from 'node:timers/promises';
import { dirname } from 'node:path';
export class ExecutionSaveError extends Error {
  constructor(cause?: unknown) { super('실행 기록 저장에 실패하여 중단했습니다. 디스크 공간과 폴더 쓰기 권한을 확인하세요.', { cause }); }
}
/** Single-writer replacement: readers see the previous or complete new JSON, not half a write. */
export async function atomicWrite(path: string, text: string): Promise<void> {
  const temporary = `${path}.${randomUUID()}.tmp`;
  try {
    await mkdir(dirname(path), { recursive: true });
    const handle = await open(temporary, 'wx', 0o600);
    try { await handle.writeFile(text, 'utf8'); await handle.sync(); } finally { await handle.close(); }
    for (let attempt = 0; ; attempt++) {
      try {
        await rename(temporary, path);
        break;
      } catch (error) {
        const code = (error as NodeJS.ErrnoException).code;
        if (process.platform !== 'win32' || !['EPERM', 'EBUSY', 'EACCES'].includes(code ?? '') || attempt >= 8) throw error;
        await delay(Math.min(500, 25 * 2 ** attempt));
      }
    }
  } catch (cause) { throw new ExecutionSaveError(cause); }
  finally { await rm(temporary, { force: true }).catch(() => {}); }
}
