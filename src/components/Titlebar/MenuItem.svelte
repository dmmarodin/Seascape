<div><span><slot /></span></div>

<style lang="scss">
    @import "../../styles/colors.scss";

    div {
        color: $gray-light;
        display: flex;
        align-items: center;
        font-size: 1rem;
        margin-right: 5px;

        span {
            height: 70%;
            padding: 8px 8px;
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
