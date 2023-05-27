import * as React from 'react';
import Box from '@mui/material/Box';
import SearchIcon from '@mui/icons-material/Search';
import LabelIcon from '@mui/icons-material/Label';
import InputBase from '@mui/material/InputBase';
import Paper from '@mui/material/Paper';
import Button from '@mui/material/Button';
import List from '@mui/material/List';
import ListItem from '@mui/material/ListItem';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Divider from '@mui/material/Divider';
import { styled, alpha } from '@mui/material/styles';
import { Container, Stack } from '@mui/material';
import { useState } from 'react';

const Search = styled('div')(({ theme }) => ({
  position: 'relative',
  borderRadius: theme.shape.borderRadius,
  backgroundColor: alpha(theme.palette.common.black, 0.15),
  '&:hover': {
    backgroundColor: alpha(theme.palette.common.black, 0.25),
  },
  marginLeft: 0,
  width: '100%',
  [theme.breakpoints.up('sm')]: {
    marginLeft: theme.spacing(1),
    width: 'auto',
  },
}));

const SearchIconWrapper = styled('div')(({ theme }) => ({
  padding: theme.spacing(0, 2),
  height: '100%',
  position: 'absolute',
  pointerEvents: 'none',
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'center',
}));

const StyledInputBase = styled(InputBase)(({ theme }) => ({
  color: 'inherit',
  '& .MuiInputBase-input': {
    padding: theme.spacing(1, 1, 1, 0),
    // vertical padding + font size from searchIcon
    paddingLeft: `calc(1em + ${theme.spacing(4)})`,
    transition: theme.transitions.create('width'),
    width: '100%',
    [theme.breakpoints.up('sm')]: {
      width: '20ch',
      '&:focus': {
        width: '20ch',
      },
    },
  },
}));

const Item = styled(Paper)(({ theme }) => ({
  backgroundColor: theme.palette.mode === 'dark' ? '#1A2027' : '#fff',
  ...theme.typography.body2,
  padding: theme.spacing(1),
  textAlign: 'center',
  color: theme.palette.text.secondary,
}));


function SearchWrapper(props) {
  const [searchQuery, setSearchQuery] = useState("");
  return (
      <Search>
        <SearchIconWrapper>
          <SearchIcon />
        </SearchIconWrapper>
        <StyledInputBase
          placeholder="Search…"
          inputProps={{ 'aria-label': 'search' }}
          onChange={(e) => setSearchQuery(e.target.value)}
          onKeyDown={(e) => {if (e.key === "Enter") {
            console.log(encodeURIComponent(searchQuery));
            props.onSearch(encodeURIComponent(searchQuery));
          }}}
        />
      </Search>
  );
}

function Tags(props) {
  return (
      props.contents.map((tag) => {
        return (
          <ListItem key={tag}>
            <ListItemButton onClick={(_) => props.onClickTag(tag)}>
              <ListItemIcon>
                <LabelIcon/>
              </ListItemIcon>
              <ListItemText primary={tag}/>
            </ListItemButton>
          </ListItem>
        );
      })
  );
}

export default function TagBar(props) {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <Container maxWidth="sm">
        <SearchWrapper onSearch={props.onSearch}/>
        <List>
          <Tags contents={props.tags} onClickTag={props.onClickTag}/>
        </List>
      </Container>
    </Box>
  );
};