<script lang="ts">
    import MenuItem from "./MenuItem.svelte";
    import LogoIcon from "../Layout/LogoIcon.svelte";
    import Controls from "./Controls.svelte";
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
        <Controls />
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
