# Jellyfin to Ryot Import Generator

**Generates JSON for importing watch Show & Movie watch data from [Jellyfin](https://jellyfin.org/) into [Ryot](https://github.com/IgnisDa/ryot)**

---

## Requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [jq](https://jqlang.github.io/jq/) (optional, makes setup easier)
- An API key for your Jellyfin instance. This can be generated at `your-jellyfin-instance.com/web/index.html#!/apikeys.html`

## Setup

1. `cargo install --git https://github.com/ellsclytn/jellyfin-ryot-import.git`
2. Set the required environment variables:

   - `JF_BASE_URL`: the URL to your Jellyfin instance in the format https://jellyfin.instance (no trailing slash)
   - `JF_API_KEY`: a Jellyfin API key
   - `JF_USER_ID`: the ID of the Jellyfin user to query. User IDs can be found with an API request, for example:

     ```sh
     curl "$JF_BASE_URL/Users" -H "X-Emby-Token: $JF_API_KEY" | jq 'map({Id, Name})'
     ```

   - `JF_TV_LIBRARY_ID`: the ID of the TV Shows library in Jellyfin. Library IDs can be queried via the API, for example:

     ```sh
     curl "$JF_BASE_URL/Users/$JF_USER_ID/Items" -H "X-Emby-Token: $JF_API_KEY" | jq '.Items |= map({Id, Name})'
     ```

   - `JF_MOVIE_LIBRARY_ID`: the same as above, but for the Movies library instead

## Usage

```
$ jellyfin-ryot-import shows > shows.json
$ jellyfin-ryot-import movies > movies.json
```

The JSON files generated can be imported in Ryot by going to Settings > Imports and choosing "Media JSON" as the source.
