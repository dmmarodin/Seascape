<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";
    import TitlebarControl from "./TitlebarControl.svelte";
    import MenuItem from "./MenuItem.svelte";
    import LogoIcon from "../Layout/LogoIcon.svelte";
</script>

<nav>
    <div data-tauri-drag-region class="titlebar">
        <div class="menu">
            <div class="logo">
                <LogoIcon size={16} />
            </div>
            <div class="menu-items">
                <MenuItem>File</MenuItem>
                <MenuItem>Edit</MenuItem>
                <MenuItem>View</MenuItem>
            </div>
        </div>
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
    </div>
</nav>
<div class="spacer" />

<style lang="scss">
    $titlebar-height: 35px;

    .titlebar {
        height: $titlebar-height;
        user-select: none;
        display: flex;
        justify-content: space-between;
        position: fixed;
        top: 0;
        left: 0;
        right: 0;
        border-bottom: 1px solid rgba($color: #fff, $alpha: 0.08);
    }

    .spacer {
        height: $titlebar-height;
    }

    .menu {
        display: flex;
        align-items: center;
        margin-left: 5px;
    }

    .menu-items {
        display: flex;
    }

    .controls {
        display: flex;
    }

    .logo {
        margin: 0 8px;
        padding: 5px;
        border-radius: 100%;
        display: flex;
        align-items: center;

        &:hover {
            background-color: rgba($color: #fff, $alpha: 0.05);
        }
    }
</style>
