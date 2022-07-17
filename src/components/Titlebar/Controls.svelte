<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import TitlebarControl from "./TitlebarControl.svelte";
</script>

<div class="controls">
    <TitlebarControl on:click={appWindow.minimize}>
        <img
            src="https://api.iconify.design/mdi:window-minimize.svg?color=%23999"
            alt="minimize"
        />
    </TitlebarControl>
    <TitlebarControl on:click={appWindow.toggleMaximize}>
        <img
            src="https://api.iconify.design/mdi:window-maximize.svg?color=%23999"
            alt="maximize"
        />
    </TitlebarControl>
    <TitlebarControl on:click={appWindow.close}>
        <img
            src="https://api.iconify.design/mdi:close.svg?color=%23999"
            alt="close"
        />
    </TitlebarControl>
</div>

<style lang="scss">
    .controls {
        display: flex;
    }
</style>
