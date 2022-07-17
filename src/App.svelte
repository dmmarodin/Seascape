<script lang="ts">
    import Titlebar from "./components/Titlebar/Titlebar.svelte";
</script>

<main>
    <Titlebar />
</main>

<style lang="scss">
    @import url("https://fonts.googleapis.com/css2?family=DM+Sans:ital,wght@0,400;0,700;1,400;1,700&display=swap");
    :global(*) {
        font-family: "DM Sans", sans-serif;
        box-sizing: border-box;
    }

    :global(body) {
        margin: 0;
    }
</style>
