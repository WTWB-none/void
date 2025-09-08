
/**
 * Copyright 2025 The VOID Authors. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
import { invoke } from "@tauri-apps/api/core";
import { Router } from "vue-router";
import { useExplorerStore } from "./explorerstore";
import { readFile, readTextFile } from "@tauri-apps/plugin-fs";

type AudioMeta = {
  picture: Uint8Array,
  title: string,
  author: string
}

type AudioResponse = {
  picture_url: string,
  author: string,
  title: string
}

export type Entry = {
  name: string,
  entry_type: string
}

export async function getFullPath(subPath: string): Promise<string> {
  return await invoke('get_absolute_path', { subpath: subPath });
}

export async function get_file_content(
  path: string,
): Promise<string> {
  //let bytes: Uint8Array = await invoke("get_file", { ipath: path });
  //let url = URL.createObjectURL(
  //  new Blob([new Uint8Array(bytes)], { type: intype }),
  //);
  const binary = await readFile(path);
  const blob = new Blob([new Uint8Array(binary)]);
  const url = URL.createObjectURL(blob);
  return url;
}

export async function get_audio_content(path: string): Promise<AudioResponse> {
  let base64_string = await invoke<AudioMeta>('read_audio_metadata', { path: path });
  const blob = new Blob([new Uint8Array(base64_string.picture)]);
  const url = URL.createObjectURL(blob);
  const responce: AudioResponse = {
    picture_url: url,
    author: base64_string.author,
    title: base64_string.title,
  }
  return responce;
}

export function checkShowable(): boolean {
  if (window.innerHeight >= window.innerWidth) {
    return false;
  } else {
    return true;
  }
}

export async function get_folder_content(dirname: string): Promise<Entry[]> {
  return await invoke<Entry[]>("get_directory_content", { dirname: dirname });
}

export async function create_file(name: string, dirname: string) {
  let res = await invoke("create_entry", { name: name, path: dirname, flag: "file" });
  console.log(res);
}

export async function create_folder(name: string, dirname: string) {
  let res = await invoke("create_entry", { name: name, path: dirname, flag: "folder" });
  console.log(res);
}
export async function delete_folder(name: string, dirname: string) {
  await invoke("remove", { name: name, path: dirname, flag: "folder" });
}

export async function delete_file(name: string, dirname: string) {
  await invoke("remove", { name: name, path: dirname, flag: "file" });
}

export async function rename(before_path: string, name: string) {
  await invoke('rename', { path: before_path, newName: name });
}

export async function decide_file_ext(name: string, router: Router) {
  console.log(name);
  let ext_map = new Map();
  ext_map.set("mp3", "audio");
  ext_map.set("wav", "audio");
  ext_map.set("ogg", "audio");
  ext_map.set("flac", "audio");
  ext_map.set("alac", "audio");
  ext_map.set("opus", "audio");
  ext_map.set("m4a", "audio");
  ext_map.set("mp4", "video");
  ext_map.set("mov", "video");
  ext_map.set("pth", "note");
  ext_map.set("md", "note");
  ext_map.set("epub", "book");
  ext_map.set("fb2", "book");
  ext_map.set("pdf", "journal");
  ext_map.set("jpg", "image");
  ext_map.set("png", "image");
  ext_map.set("webp", "image");
  ext_map.set("gif", "image");
  ext_map.set("canvas", "canvas");
  let extension = name.split('.')[name.split('.').length - 1];
  if (ext_map.get(extension) != undefined) {
    let explorer = useExplorerStore();
    name = name.replaceAll(' ', '\ ');
    let file_path = explorer.current + name;
    let coded_path = btoa(encodeURIComponent(file_path));
    router.push('/' + ext_map.get(extension) + '/' + coded_path);
    console.log(file_path);
  }
  else {
    router.push('/unsupported');
  }
}

export const SettingsRegistry = ['Global', 'Plugins', 'SideRepos'];

export async function write_canvas(data: string, path?: string): Promise<string> {
  let npath = path == undefined ? '' : '/' + path;
  return await invoke("write_canvas_data", { path: npath, data: data });
}

export async function read_canvas(path: string): Promise<string> {
  let data = await readTextFile(path);
  return data;
}

export async function get_env(key: string): Promise<string> {
  return await invoke('get_env', { ename: key });
}

export async function copy_entry(before_path: string, after_path: string, flag: string) {
  await invoke('modify_entry', { beforePath: before_path, afterPath: after_path, flag: flag });
}
