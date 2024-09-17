<script context="module">

    import {writable} from 'svelte/store';

    // Which filter is currenly opened
    export let open_filter = writable("");

    // The results are sorted by
    export let sort_by = writable("alphanumeric");

</script>
