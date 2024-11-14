<script lang='ts'>
    import { get } from "svelte/store";
    import { activeState } from "../../store";
    let showDiv = $state(false);
    function clicked(){
        if (get(activeState) === 'profile'){
            activeState.set(null);
            showDiv = false;
        } else {
            activeState.set('profile');
            showDiv = true;
        }
    }

</script>

<button class="selection" onclick={clicked} disabled={$activeState && $activeState !== 'profile' ? true : null}>    
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