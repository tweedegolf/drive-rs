<script lang="ts">
    import type {FullCrate, Indexes} from "../crate-db";
    import Fuse, {type FuseResult} from "fuse.js";

    export let indexes: Indexes;

    let search: string = "";

    const {
        category,
        has_dev_board,
        has_kicad,
        license,
        interfaces,
        rust_version,
        dependencies,
        package: pack,
        ...rest
    } = indexes;
    const _is_exhaustive: Required<typeof rest> = {};

    const items = [
        {"value": "kicad", "filter": "has_kicad"},
        {"value": "dev board", "filter": "has_dev_board"},
        {"value": "rust version", "filter": "rust_version"}
    ].concat(
        Object.keys(category).map((k) => ({value: k, filter: "category"})),
        Object.keys(license).map((k) => ({value: k, filter: "license"})),
        Object.keys(interfaces).map((k) => ({value: k, filter: "interfaces"})),
        Object.keys(dependencies).map((k) => ({value: k, filter: "dependencies"})),
        Object.keys(pack).map((k) => ({value: k, filter: "pack"})),
    );

    const options = {
        keys: ["value", "filter"],
    };
    const fuse = new Fuse(items, options)

    type FuseResultList = FuseResult<{ filter: string, value: string }>[];

    function onlyTop(results: FuseResultList, count: number): FuseResultList {
        let seen: any = {};
        return results.filter((v) => {
            if (seen[v.item.filter] == undefined) {
                seen[v.item.filter] = 1;
            } else {
                seen[v.item.filter] += 1;
            }

            return seen[v.item.filter] <= count;
        });
    }

    $: results = fuse.search(search)
    $: top_results = onlyTop(results, 3)
</script>

<label>
    Search Category:
    <input type="text" bind:value={search}>
</label>

<ul>
    {#each top_results as res}
        <li>{res.item.filter}: {res.item.value}</li>
    {/each}
</ul>