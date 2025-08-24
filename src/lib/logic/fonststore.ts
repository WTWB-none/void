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

import { defineStore } from "pinia";
import { get_file_content } from "./utils";
import { invoke } from "@tauri-apps/api/core";
export const useFontStore = defineStore('font', {
  state: () => ({ current: localStorage.getItem('mindbreaker:font') || 'Spectral' }),
  actions: {
    async loadFont() {
      console.warn('penis');
      if (this.current != 'Spectral') {
        let font_path = await invoke('get_app_config_dir') + '/fonts/' + this.current
        console.log(font_path);
        let file = await get_file_content(font_path);
        console.log(file);
        let font_face = new FontFace('MyFont', `url(${file})`);
        await font_face.load();
        document.fonts.add(font_face);
        document.body.style.fontFamily = 'MyFont, system-ui, sans-serif';
      } else {
        document.body.style.fontFamily = 'Spectral, system-ui, sans-serif';
      }
    },
    changeFont(font: string) {
      localStorage.setItem('mindbreaker:font', font);
      this.current = font;
    }
  }
})
