<script lang="ts">
    import Headerbar from "$lib/components/Headerbar.svelte";


</script>


<Headerbar loader_data={["current_user", "user"]}/>
<main>
    <div class="col side">
        <button>[Tink anmelden wenn kein token da ist]</button>
    </div>
    <div class="col center">
        <span>[Tink Menü wenn token da ist]</span>
        <span>Dann &rarr; [Geladene payments]</span>
    </div>
    <div class="col side">
        <span>[Fehler, Info, Zusammenfassung, etc]</span>
        <button>[Abschicken (ConfirmButton)]</button>
    </div>
</main>

<style>

</style>