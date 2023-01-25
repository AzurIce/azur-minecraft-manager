<script lang="ts">
  import { updateModFiles } from "$lib/apis/mod_file";
  import RemoteModCard from "$lib/components/RemoteModCard.svelte";
  import LocalModFileCard from "$lib/components/LocalModFileCard.svelte";
  import { modFiles, targetDir } from "$lib/stores";
  import { invoke } from "@tauri-apps/api";
  import { Button, Spinner, TabItem, Tabs } from "flowbite-svelte";
  import { onMount } from "svelte";
  import { getVersionFromHash } from "$lib/apis/version";
  import { join } from "@tauri-apps/api/path";
  import { updateProjects } from "$lib/apis/project";

  enum Tab {
    RemoteMod = "远端 Mod",
    LocalMod = "本地 Mod",
  }
  let selectedTab = Tab.RemoteMod;
  $: tabs = Object.values(Tab);

  let remote_mods: Array<string> = new Array();
  $: invoke<Array<string>>("get_remote_mods", {
    targetDir: $targetDir,
  }).then((res) => {
    remote_mods = res;
  });

  let localModFiles: Array<any> = new Array();
  let remoteModFiles: Array<any> = new Array();
  $: remoteModFiles = $modFiles.filter(
    (modFile) =>
      modFile.remote_version != undefined &&
      remote_mods.includes(modFile.remote_version.project_id)
  );
  $: localModFiles = $modFiles.filter(
    (modFile) => !remoteModFiles.includes(modFile)
  ).sort((a, b) => a.filename < b.filename ? -1 : 1);
  // $: console.log(remoteModFiles);

  let loading = true;
  onMount(async () => {
    console.log("onMount -> /main");
    console.log("    updating mod files...");

    let _modFiles = await updateModFiles($targetDir);
    await Promise.all(
      _modFiles.map(async (modFile, i) => {
        try {
          modFile.remote_version = await getVersionFromHash(modFile.sha1);
        } catch (err) {
        } finally {
          $modFiles = [...$modFiles, modFile];
        }
      })
    );
    loading = false;
  });

  let updating = false;
</script>

<div class="flex flex-col flex-1 bg-white p-4 gap-2">
  <Button
    disabled={updating}
    on:click={() => {
      console.log("updating...");
      updating = true;
      updateProjects(remote_mods)
        .then((res) => {
          console.log("updated: ", res);
          updating = false;
          remote_mods = remote_mods;
        })
        .catch((err) => {
          console.log(err);
          updating = false;
        });
    }}
    >{#if updating} <Spinner size={4} />{/if}更新 Modrinth 数据</Button
  >
  <Tabs contentClass="">
    {#each tabs as tab, i}
      <TabItem
        open={i == 0}
        title={tab}
        on:click={() => {
          selectedTab = tab;
        }}
      />
    {/each}
  </Tabs>

  <!-- Tab Contents -->
  {#if selectedTab == Tab.RemoteMod}
    <div class="w-full overflow-y-auto flex flex-col gap-1">
      <!-- {#if remote_mods.length > 0}
        <RemoteModCard project_id={remote_mods[0]} />
      {/if} -->
      {#each remote_mods as project_id}
        <RemoteModCard {project_id} />
      {/each}
    </div>
  {:else if selectedTab == Tab.LocalMod}
    <div class="w-full bg-white border rounded-md p-2">
      {loading}
    </div>
    <div
      class="w-full overflow-y-auto flex flex-col flex-1 gap-1 items-stretch"
    >
      {#each localModFiles as file (file.sha1)}
        <LocalModFileCard
          bind:file
          on:addToRemoteMod={async (e) => {
            const project_id = e.detail.project_id;
            const version_id = e.detail.version_id;

            try {
              await Promise.all([
                invoke("add_remote_mod", {
                  targetDir: $targetDir,
                  projectId: project_id,
                }),
                invoke("copy_file", {
                  src: await join(
                    $targetDir,
                    ".minecraft",
                    "mods",
                    file.filename
                  ),
                  dst: await join(
                    $targetDir,
                    ".amcm",
                    "storage",
                    project_id,
                    `${version_id}.jar`
                  ),
                }),
              ]);
              console.log("added remote mod and copied file");
              remote_mods.push(project_id);

              localModFiles = localModFiles.filter((modFile) => {
                if (
                  modFile.remote_version != undefined &&
                  modFile.remote_version.project_id == project_id
                ) {
                  remoteModFiles.push(modFile);
                  return false;
                }
                return true;
              });
            } catch (err) {
              console.log(err);
            }
          }}
        />
      {/each}
      {#if loading}
        <div class="m-auto mt-2 mb-2">
          <Spinner />
        </div>
      {/if}
    </div>
  {/if}
</div>
