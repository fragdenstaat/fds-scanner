import { invoke } from '@tauri-apps/api/core';

export interface WebAuthArguments {
  url: String
  redirectUrl: String
}

export interface WebAuthResult {
  url: String
}

export async function startWebAuth(args: WebAuthArguments): Promise<String | null> {
  return await invoke<WebAuthArguments>('plugin:webauth|start_auth', {
    payload: args,
  }).then((r) => (r ? r.url : null));
}
