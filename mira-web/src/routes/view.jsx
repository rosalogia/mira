import * as React from "react";
import Grid from "@mui/material/Grid";
import PreviewGrid from "../components/PreviewGrid";
import TagBar from "../components/TagBar";
import { useState, useEffect } from "react";

async function fetchPost(id) {
  return fetch("http://localhost:3000/api/browse/view/" + id).then((response) =>
    response.json()
  );
}

export default function Browse() {
  const [posts, setPosts] = useState([]);
  const [tags, setTags] = useState([]);
  const [searchQuery, setSearchQuery] = useState("");

  const fetchPosts = async (endpoint) => {
    console.log("Requesting http://localhost:3000/api" + endpoint);
    const allPosts = await fetch("http://localhost:3000/api" + endpoint);
    const allPostsData = await allPosts.json();
    const postPromises = allPostsData.map(async (post) => {
      const postInfo = await fetchPost(post.id);
      return postInfo.tags;
    });
    const postTags = await Promise.all(postPromises);
    const combinedTags = new Set(postTags.flat());
    setPosts(allPostsData);
    setTags(Array.from(combinedTags).sort());
  };

  useEffect(() => {
    fetchPosts("/browse/view");
  }, []);

  useEffect(() => {
    if (searchQuery === "") {
      fetchPosts("/browse/view");
    } else {
      fetchPosts(`/browse/search?tags=${searchQuery.replace(/, /g, ",")}`);
    }
  }, [searchQuery]);

  return (
    <Grid container>
      <Grid xs={2}>
        <TagBar
          tags={tags}
          onSearch={(q) => setSearchQuery(q)}
          onClickTag={(t) => setSearchQuery(t)}
        />
      </Grid>
      <Grid xs={10}>
        <PreviewGrid posts={posts} />
      </Grid>
    </Grid>
  );
}
