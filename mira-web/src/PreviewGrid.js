import * as React from 'react';
import Grid from '@mui/material/Grid';

export default function PreviewGrid(props) {
    return (
        <Grid className="PreviewGrid" container spacing={1}>
            {props.posts.map((post) => (
                <Grid key={post.id} xs={1.2}>
                    <a href={`http://localhost:3000/api/browse/view/${post.id}`}><img src={`http://localhost:3000/static/${post.img_path}`} width={200} /></a>
                </Grid>
            ))}
        </Grid>
    );
};