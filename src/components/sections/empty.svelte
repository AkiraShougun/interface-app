<script lang="ts">
  import { get } from "svelte/store";
    import { activeState } from "../../store";
    let props = $props();
    function clicked(){
        if (get(activeState) === props.text){
            activeState.set(null);
        } else {
            activeState.set(props.text);
        }
    }
</script>
<button class="selection" onclick={clicked} disabled={$activeState && $activeState !== props.text ? true : null}>    
    <h1>{props.text}</h1>
</button>

<style>
    @import './section.css';
</style>