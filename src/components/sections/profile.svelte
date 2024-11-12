<script lang='ts'>
    let showDiv = false;
    function toggleDiv() {
        showDiv = !showDiv;
    }

</script>

<button class="selection" onclick={toggleDiv}>    
    <h1>P</h1>
</button>

{#if showDiv}
    <div class="profile-content">
        <h1>Profile</h1>
        <p>Profile content</p>
    </div>
{/if}

<style>
    @import './section.css';
    .profile-content {
        position: absolute;
        left: 10rem;
        top: 8rem;
        height: 50vh;
        width: 200px;
        padding-right: 30px;
        background-color: antiquewhite;
    }
</style>