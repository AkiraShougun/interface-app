<script lang='ts'>
    import {  invoke } from '@tauri-apps/api/core';
    import { get } from "svelte/store";
    import { activeState } from "../../store";

    let showDiv = $state(false);
    let SysInfo:Array<string> = $state([]);
    async function fetchSystemInfo(){
        const response = await invoke<string[]>('get_system_info');
        SysInfo = response;
    }

    function clicked(){
        if (get(activeState) === 'profile'){
            activeState.set(null);
            showDiv = false;
        } else {
            activeState.set('profile');
            fetchSystemInfo();
            showDiv = true;
        }
    }

</script>

<button class:active={$activeState == 'profile'} class="selection" onclick={clicked} disabled={$activeState && $activeState !== 'profile' ? true : null}>    
    <h1>P</h1>
</button>

{#if showDiv}
    <div class="profile-content">
        <h1>User: {SysInfo[0]}</h1>
        <h1>OS: {SysInfo[1]}</h1>
        <h1>Battery: {SysInfo[2]}%</h1>
    </div>
{/if}

<style>
    @import './section.css';
    .profile-content {
        position: absolute;
        left: -15rem;
        height: 50vh;
        width: 200px;
        padding-right: 30px;
        background-color: #1D2D47;
        border: 1px solid rgb(45, 169, 169);
    }
</style>