<div on:click>
    <span>
        <slot />
    </span>
</div>

<style lang="scss">
    div {
        display: inline-flex;
        justify-content: center;
        align-items: center;
        margin: 0 4px;

        span {
            height: 70%;
            padding: 0 5px;
            border-radius: 5px;
            display: flex;
            align-items: center;
            cursor: pointer;

            &:hover {
                background-color: rgba($color: #fff, $alpha: 0.05);
            }
        }
    }
</style>
