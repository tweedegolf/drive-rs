<script lang="ts">
    import type {FullCrate} from "../crate-db";
    import Fuse from "fuse.js";

    export let crates: FullCrate[];
    export let selected: number[][];

    let search: string = "";

    const options = {
        keys: [
            "description",
            "manufacturer",
            "name",
            "names",
            "part_numbers",
            "resources.title",
        ],
        includeMatches: true,
        includeScore: true,
        useExtendedSearch: true,
    };
    const index = Fuse.createIndex(options.keys, crates)
    const fuse = new Fuse(crates, options, index)

    $: results = fuse.search(search)
    $: selected = search === "" ? [] : [results.map((r) => r.refIndex)]
</script>

<div>
    <input type="text" bind:value={search}>
</div>