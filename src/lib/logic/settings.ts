
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
import { Router } from "vue-router";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useThemeStore } from "./themestore";
import { Component } from "vue";

export type Theme = {
  theme_name: string,
  theme_author: string,
  theme_version: string,
  theme_link: string,
  is_installed: string,
}

export type SideRepo = {
  repo_type: string,
  link: string,
}

export function showSettings(router: Router) {
  router.push('/settings');
}

export async function changeWorkdir(): Promise<string> {
  let error: string = '';
  const directory: string | null = await open({
    directory: true,
    multiple: false,
  });
  if (directory === null) {
    error = "path not provided";
    return error;
  }
  await invoke('set_env', { ename: 'workdir', name: directory });
  await invoke('setup_config_directory')
  localStorage.setItem('mindbreaker:explorer', '');
  return directory;
}

export async function getWorkdir(): Promise<string> {
  const directory: string = await invoke('get_env', { ename: 'workdir' });
  return directory;
}

export async function getUsername(): Promise<string> {
  const uname: string = await invoke('get_env', { ename: 'name' });
  return uname;
}

export async function set_theme(theme_name: string) {
  const theme_store = useThemeStore();

  const existing = document.getElementById("custom_theme");
  if (existing) existing.remove();

  if (theme_name !== 'lotm') {
    const css = await invoke<string>("get_theme", { name: theme_name });

    const style_el = document.createElement('style');
    style_el.id = "custom_theme";
    style_el.innerHTML = css;
    document.head.appendChild(style_el);
  }

  theme_store.change_theme(theme_name);
}

export function get_themes_marketplace(object: any[]): any[] {
  return object;
}

export function closePopup(event: any): boolean {
  if (event.target == event.currentTarget) {
    return false;
  }
  else {
    return true;
  }
}

export async function importComponent(name: string): Promise<Component> {
  let component;
  if (name != 'Global') {
    component = await import('./' + name + '.vue');
  }
  return component;
}


export async function get_installed_themes_list(): Promise<string[]> {
  let themes_arr: Theme[] = await invoke("get_list_of_themes", { key: 'installed' });
  console.log(themes_arr);
  let names_arr: string[] = [];
  themes_arr.forEach((theme) => {
    names_arr.push(theme.theme_name);
  });
  return names_arr
}

export async function get_themes_to_download(): Promise<Theme[]> {
  let theme_arr: Theme[] = await invoke("get_list_of_themes", { key: 'not_installed' });
  return theme_arr
}

export async function add_themes_repo(link: string) {
  link = link.replace('https://', '');
  await invoke('create_themes_table', { link: link });
}

export async function get_repos(): Promise<SideRepo[]> {
  let repos: any[] = await invoke("get_repos_list");
  console.log(repos);
  return repos;
}

export async function delete_repo(link: string) {
  let res = await invoke("delete_repo", { link: link });
  console.log(res);
}

export async function update(name: string) {
  await invoke("check_theme_update", { themeName: name });
}

export async function delete_theme(name: String) {
  await invoke("delete_theme", { themeName: name });
}

export async function addCustomFont() {
  let font = await open({ multiple: false, filters: [{ name: 'Font', extensions: ['ttf', 'otf'] }] });
  await invoke('copy_font', { path: font });
}

export async function get_user_fonts(): Promise<string[]> {
  return await invoke('get_all_user_fonts');
}
