import {writable} from 'svelte/store';

export interface Score {
  name: String;
  score: number;
}

// Which filter is currenly opened
export let open_filter = writable("");

// The results are sorted by
export let sort_by = writable("alphanumeric");

// Results of text search
export let scores = writable<Score[]>([]);

