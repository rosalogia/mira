import logo from './logo.svg';
import my_image from './43c888064127e930b570669b9fffb166310b8d5e.jpg';
import './App.css';
import * as React from 'react';
import Grid from '@mui/material/Grid';
import NavBar from './NavBar';
import PreviewGrid from './PreviewGrid';
import TagBar from './TagBar';
import { useState, useEffect } from 'react';

async function fetchPost(id) {
  return fetch("http://localhost:3000/api/browse/view/" + id)
    .then(response => response.json())
}


function getTags(posts) {
  var s = new Set();
  posts.forEach((p) => {
    fetchPost(p.id)
      .then((p_r) => p_r.tags.forEach((tag) => s.add(tag)))
      .catch((e) => { console.log(e); })
  });
  // console.log(s);
  return s;
}

function App() {
  const [posts, setPosts] = useState([]);
  const [tags, setTags] = useState([]);
  const [searchQuery, setSearchQuery] = useState("");

  const fetchPosts = async (endpoint) => {
    console.log("Requesting http://localhost:3000/api" + endpoint);
    const allPosts = await fetch("http://localhost:3000/api" + endpoint);
    const allPostsData = await allPosts.json();
    const postPromises = allPostsData.map(async (post) => {
      const postInfo = await fetchPost(post.id);
      return postInfo.tags
    });
    const postTags = await Promise.all(postPromises);
    const combinedTags = new Set(postTags.flat());
    setPosts(allPostsData);
    setTags(Array.from(combinedTags).sort());
  };

  useEffect(() => {
    fetchPosts("/browse/view");
  }, [])

  useEffect(() => {
    if (searchQuery === "") {
      fetchPosts("/browse/view")
    } else {
      fetchPosts(`/browse/search?tags=${searchQuery.replace(/, /g, ',')}`)
    }
  }, [searchQuery]);

  return (
    <div className="App">
      <NavBar>
        <Grid container>
          <Grid xs={2}>
            <TagBar tags={tags} onSearch={(q) => setSearchQuery(q)} onClickTag={(t) => setSearchQuery(t)}/>
          </Grid>
          <Grid xs={10}>
            <PreviewGrid posts={posts} />
          </Grid>
        </Grid>
      </NavBar>
    </div>
  );
}

export default App;
