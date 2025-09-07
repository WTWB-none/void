
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
export const useExplorerStore = defineStore('explorer', {
  state: () => ({ current: localStorage.getItem('mindbreaker:explorer') || '' }),
  actions: {
    add_path(folder: string) {
      this.current += folder + "/";
      localStorage.setItem('mindbreaker:explorer', this.current)
    },
    remove_path() {
      let path_to_remove = this.current.split('/')[this.current.split("/").length - 2];
      if (path_to_remove == '') {
        this.current = '';
      } else {
        this.current = this.current.replace(path_to_remove + '/', "");
        console.log(this.current);
      }
      localStorage.setItem('mindbreaker:explorer', this.current)
    }
  }
})
