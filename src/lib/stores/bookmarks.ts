import { writable } from 'svelte/store';

export type Bookmark = {
  repo_name: string;
  repo_url: string;
};

function createBookmarks() {
  const { subscribe, set, update } = writable<Bookmark[]>([
    { repo_name: "fit3170-A1", repo_url: "https://github.com/user/fit3170-A1.git" },
    { repo_name: "this-is-a-repo", repo_url: "https://gitlab.com/abc0012/this-is-a-repo.git" },
    { repo_name: "project", repo_url: "https://github.com/example-org/project.git" },
  ]);

  return {
    subscribe,
    add: (bookmark: Bookmark) =>
      update(bookmarks => {
        if (!bookmarks.find(b => b.repo_url === bookmark.repo_url)) {
          return [...bookmarks, bookmark];
        }
        return bookmarks;
      }),
    remove: (repo_url: string) =>
      update(bookmarks => bookmarks.filter(b => b.repo_url !== repo_url)),
    toggle: (bookmark: Bookmark) =>
      update(bookmarks => {
        const exists = bookmarks.find(b => b.repo_url === bookmark.repo_url);
        if (exists) {
          return bookmarks.filter(b => b.repo_url !== bookmark.repo_url);
        } else {
          return [...bookmarks, bookmark];
        }
      }),
    clear: () => set([]),
  };
}

export const bookmarks = createBookmarks();
