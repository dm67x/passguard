import { render } from 'react-dom'
import React from 'react'
import { HashRouter, Route, Switch } from 'react-router-dom'
import WelcomeForm from './components/WelcomeForm'
import PasswordTable from './components/PasswordTable'
import { Container } from '@material-ui/core'
import MainContext from './components/MainContext'

const App = () => {
    return (
        <HashRouter>
            <MainContext.Provider value={{ username: null }}>
                <Container fixed>
                    <Switch>
                        <Route path="/" exact component={WelcomeForm} />
                        <Route path="/passwords" component={PasswordTable} />
                    </Switch>
                </Container>
            </MainContext.Provider>
        </HashRouter>
    )
}

render(<App />, document.getElementById("root"))
