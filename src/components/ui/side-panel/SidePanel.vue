<script setup lang="ts">
import Sidebar from "../sidebar/Sidebar.vue";
import SidebarHeader from "../sidebar/SidebarHeader.vue";
import SidebarFooter from "../sidebar/SidebarFooter.vue";
import SidebarMenuButton from "../sidebar/SidebarMenuButton.vue";
import { Folder, Settings } from "lucide-vue-next";
import SidebarMenuItem from "../sidebar/SidebarMenuItem.vue";
import SidebarMenu from "../sidebar/SidebarMenu.vue";
import SidebarGroupContent from "../sidebar/SidebarGroupContent.vue";
import SidebarContent from "../sidebar/SidebarContent.vue";
import SidebarGroupLabel from "../sidebar/SidebarGroupLabel.vue";
import { showSettings } from "@/lib/logic/settings";
import { pluginRegistry } from "@/components/ui/side-panel/side-panel-items/index";
import { defineAsyncComponent, onMounted, ref } from "vue";
import { CollapsibleContent, TooltipRoot } from "reka-ui";
import { CollapsibleTrigger } from "reka-ui";
import SidebarMenuSubItem from "../sidebar/SidebarMenuSubItem.vue";
import SidebarMenuSub from "../sidebar/SidebarMenuSub.vue";
import { create_file, create_folder, delete_folder, delete_file, get_folder_content, decide_file_ext, rename, get_env, copy_entry } from "@/lib/logic/utils";
import { CollapsibleRoot } from "reka-ui";
import { useExplorerStore } from "@/lib/logic/explorerstore";
import TooltipProvider from "../tooltip/TooltipProvider.vue";
import TooltipContent from "../tooltip/TooltipContent.vue";
import TooltipTrigger from "../tooltip/TooltipTrigger.vue";
import ExplorerMenu from "./side-panel-items/ExplorerMenu.vue";
import { RecycleScroller } from 'vue-virtual-scroller';
import { useSidebar } from "../sidebar";
import { watch } from "vue";
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';
import { useSidebarStore } from "@/lib/logic/sidebarstore";
import { nanoid } from 'nanoid';
import { Input } from '@/components/ui/input';
import { watchImmediate as watchDir } from "@tauri-apps/plugin-fs";
import { Entry } from '@/lib/logic/utils';
const plugins = pluginRegistry;
const loadedPlugins = plugins.reduce((acc: any, name) => {
  acc[name] = defineAsyncComponent(() => import(`@/components/ui/side-panel/side-panel-items/${name}.vue`));
  return acc;
}, {});

let entries = ref<{
  id: any, name: string, type: string
}[]>([]);
let fcreate = ref(false);
let dcreate = ref(false);
let expanded = ref(false);
let create_type = ref("");
let name = ref("");
let rename_mode = ref({ name: '', enabled: false });
let copied = ref('');
let paste_flag = ref('');

const { state } = useSidebar();
const explorer_store = useExplorerStore();
const sidebar_store = useSidebarStore();
const sidebar_state = useSidebar();

watch(state, (v) => { if (v === 'collapsed') expanded.value = false; });

onMounted(async () => {
  let watchdir = await get_env('workdir');
  await watchDir(watchdir, async () => {
    await strip_content();
  }, {
    recursive: true
  });
  window.addEventListener('keydown', (event) => {
    if (event.metaKey && event.key == 's') {
      event.preventDefault();
      sidebar_store.toggle();
      sidebar_state.toggleSidebar();
    }
  })
  await strip_content();
});


async function modify_store(dir: string) {
  dir !== '..' ? explorer_store.add_path(dir) : explorer_store.remove_path();
  await strip_content();
}

function performCreation(flag: string) {
  switch (flag) {
    case 'file':
      fcreate.value = true;
      create_type.value = 'file';
      break;

    case 'folder':
      dcreate.value = true;
      create_type.value = 'folder';
      break;
  }
}


async function summon(flag: string) {
  let folder = explorer_store.current;
  if ((fcreate.value || dcreate.value) && name.value != "") {
    switch (flag) {
      case 'file':
        await create_file('/' + name.value, folder);
        create_type.value = "";
        name.value = "";
        fcreate.value = false;
        await strip_content();
        break;
      case 'folder':
        await create_folder('/' + name.value, folder);
        create_type.value = "";
        name.value = "";
        dcreate.value = false;
        await strip_content();
        break;
    }
  }
}

async function remove_dir(name: string) {
  await delete_folder('/' + name, useExplorerStore().current);
  await strip_content();
}

async function remove_file(name: string) {
  await delete_file('/' + name, useExplorerStore().current);
  await strip_content();
}


async function strip_content() {
  let explorer_store = useExplorerStore();
  let rentries: Entry[] = await get_folder_content(explorer_store.current);
  let temp = [];

  if (explorer_store.current !== "") {
    temp.push({ id: nanoid(), name: '..', type: 'dir' });
  }

  rentries.forEach((entrie) => {
    temp.push({
      id: nanoid(),
      name: entrie.name,
      type: entrie.entry_type
    });
  });

  entries.value = temp.sort((a, b) => {
    if (a.type === b.type) return a.name.localeCompare(b.name);
    return a.type === 'dir' ? -1 : 1;
  });
}

function enter_rename(name: string) {
  rename_mode.value = {
    name: name,
    enabled: true
  };
  window.addEventListener('keydown', (event) => {
    if (event.key == 'enter') {
      event.preventDefault();
    }
  })
}

function initiate_copy(name: string) {
  copied.value = explorer_store.current + '/' + name;
}
</script>

<template>
  <Sidebar collapsible="icon" variant="floating" class="h-[95vh] mt-10 text-sidebar-primary select-none">
    <SidebarHeader />
    <SidebarContent class="px-2">
      <SidebarGroupContent>
        <SidebarMenu>
          <CollapsibleRoot :default-open="false" v-bind:open="expanded" class="group/collapsible">
            <SidebarMenuItem>
              <CollapsibleTrigger asChild>
                <SidebarMenuButton>
                  <Folder />
                  <span class="text-lg cursor-pointer" @click="expanded = !expanded">{{ $t('common.explorer') }}</span>
                </SidebarMenuButton>
              </CollapsibleTrigger>
              <ExplorerMenu :copied="copied != ''"
                @epaste="async () => { await copy_entry(copied, explorer_store.current, paste_flag); copied = ''; paste_flag = ''; }"
                @create-file="performCreation('file')" @create-folder="performCreation('folder')">
                <CollapsibleContent v-if="expanded"
                  class="mr-5 data-[state=open]:min-h-[15rem] max-h-[15rem] overflow-y-scroll">
                  <SidebarMenuSub v-if="fcreate || dcreate">
                    <SidebarMenuSubItem>
                      <span class="text-sm flex gap-1 items-center">
                        <Input type="text" v-model="name" placeholder="unnamed" @keydown="(event: any) => {
                          if (event.key == 'Enter') {
                            summon(create_type);
                          }
                          else if (event.key == 'Escape') {
                            fcreate = false;
                            dcreate = false;
                            name = '';
                          }
                        }" />
                      </span>
                    </SidebarMenuSubItem>
                  </SidebarMenuSub>
                  <RecycleScroller class="scroller h-[15rem]" :items="entries" :item-size="24" key-field="id"
                    :key="explorer_store.current" v-slot="{ item }">
                    <ExplorerMenu v-if="item.type == 'dir'" :copied="copied != ''"
                      @create-file="performCreation('file')" @create-folder="performCreation('folder')"
                      @delete="async () => await remove_dir(item.name)" @rename="enter_rename(item.name)"
                      @cut="() => { initiate_copy(item.name); paste_flag = 'move'; }"
                      @copy="() => { initiate_copy(item.name); paste_flag = 'file'; }"
                      @epaste="async () => { await copy_entry(copied, explorer_store.current, paste_flag); copied = ''; paste_flag = '' }"
                      class="h-6">
                      <SidebarMenuSub>
                        <SidebarMenuSubItem>
                          <span class="text-sm cursor-pointer flex gap-1 items-center select-none"
                            @click="async () => { if (!rename_mode.enabled) { await modify_store(item.name) } }">
                            <TooltipRoot>
                              <TooltipProvider>
                                <Folder :size="15" />
                                <TooltipTrigger>
                                  <span v-if="!rename_mode.enabled || !(rename_mode.name == item.name)"
                                    class="truncate block max-w-[10rem]">{{ item.name
                                    }}</span>
                                  <span v-else>
                                    <Input class="h-5" v-model="name" type="text" @keydown="async (event: any) => {
                                      if (event.key == 'Escape') {
                                        event.preventDefault();
                                        rename_mode = { name: '', enabled: false };
                                        name = '';
                                      }
                                      else if (event.key == 'Enter') {
                                        let current_dir = explorer_store.current;
                                        await rename(current_dir + '/' + item.name, name);
                                        rename_mode = { name: '', enabled: false };
                                        name = '';
                                        await strip_content();
                                      }
                                    }" />
                                  </span>
                                </TooltipTrigger>
                                <TooltipContent>{{ item.name }}</TooltipContent>
                              </TooltipProvider>
                            </TooltipRoot>
                          </span>
                        </SidebarMenuSubItem>
                      </SidebarMenuSub>
                    </ExplorerMenu>
                    <ExplorerMenu v-else @create-file="performCreation('file')" :copied="copied != ''"
                      @create-folder="performCreation('folder')" @delete="async () => await remove_file(item.name)"
                      @rename="enter_rename(item.name)" class="h-6" @copy="initiate_copy(item.name)"
                      @cut="() => { initiate_copy(item.name); paste_flag = 'move'; }"
                      @epaste="async () => { await copy_entry(copied, explorer_store.current, paste_flag); copied = ''; paste_flag = ''; }">
                      <SidebarMenuSub>
                        <SidebarMenuSubItem>
                          <span v-if="!rename_mode.enabled || !rename_mode.name.includes(item.name)"
                            class="text-sm cursor-pointer flex gap-1">
                            <TooltipRoot>
                              <TooltipProvider>
                                <TooltipTrigger>
                                  <span draggable="true" class="truncate block max-w-[10rem]"
                                    @click="() => decide_file_ext(item.name, $router)">{{ item.name }}</span>
                                </TooltipTrigger>
                                <TooltipContent>{{ item.name }}</TooltipContent>
                              </TooltipProvider>
                            </TooltipRoot>
                          </span>
                          <span v-else>
                            <Input class="h-5" v-model="name" type="text" @keydown="async (event: any) => {
                              if (event.key == 'Escape') {
                                event.preventDefault();
                                rename_mode = { name: '', enabled: false };
                                name = '';
                              }
                              else if (event.key == 'Enter') {
                                let current_dir = explorer_store.current;
                                let new_name = name + '.' + item.name.split('.')[item.name.split('.').length - 1];
                                await rename(current_dir + '/' + item.name, new_name);
                                rename_mode = { name: '', enabled: false };
                                name = '';
                                await strip_content();
                              }
                            }" />
                          </span>
                        </SidebarMenuSubItem>
                      </SidebarMenuSub>
                    </ExplorerMenu>
                  </RecycleScroller>
                </CollapsibleContent>
              </ExplorerMenu>
            </SidebarMenuItem>
          </CollapsibleRoot>
        </SidebarMenu>
      </SidebarGroupContent>
      <SidebarGroupLabel class="select-none">{{ $t('common.tools') }}</SidebarGroupLabel>
      <SidebarGroupContent>
        <SidebarMenu v-for="plugin in plugins" :key="plugin">
          <SidebarMenuItem>
            <SidebarMenuButton asChild>
              <component :is="loadedPlugins[plugin]" />
            </SidebarMenuButton>
          </SidebarMenuItem>
        </SidebarMenu>
      </SidebarGroupContent>
    </SidebarContent>
    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton class="cursor-pointer select-none" asChild>
            <a @click="showSettings($router)">
              <Settings />
              <span class="text-lg">{{ $t('common.settings') }}</span>
            </a>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>
  </Sidebar>
</template>
