import { Fab, TextField } from '@material-ui/core';
import { purple } from '@material-ui/core/colors';
import { createMuiTheme } from '@material-ui/core/styles';
import CreateIcon from '@material-ui/icons/Create';
import { ThemeProvider } from '@material-ui/styles';
import React from 'react';
import './App.css';

class TextBox extends React.PureComponent {
  constructor(props) {
    super(props);

    this.state = {
      content: ""
    }

    this.handleChange = this.handleChange.bind(this);
    this.send = this.send.bind(this);
  }

  handleChange(event) {
    this.setState({ content: event.target.value });
  }

  send() {
    fetch("http://localhost:8000/upload", { method: "POST", body: this.state.content })
      .then(data => data.text())
      .then(url => window.location.assign(url));
  }

  render() {
    return (
      <div classname="TextBox">
        <form>
          <TextField
            label="Paste"
            placeholder="Enter some text here!"
            variant="filled"
            multiline
            rows={15}
            value={this.state.content}
            onChange={this.handleChange}
            spellCheck={false}
            style={{
              width: 825,
              input: {
                color: "white"
              }
            }}
          />
          <p />
          <Fab
            variant="extended"
            color="primary"
            size="big"
            onClick={this.send}
          >
            <CreateIcon />&nbsp;
            Post
          </Fab>
        </form>
      </div>
    )
  }
}

const darkTheme = createMuiTheme({
  palette: {
    type: "dark",
    primary: {
      main: purple[500]
    },
  },
})

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <h1 style={{ fontFamily: "Roboto" }}>PasteBin</h1>
        <ThemeProvider theme={darkTheme}>
          <TextBox />
        </ThemeProvider>
      </header>
    </div>
  );
}

export default App;
