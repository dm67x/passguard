import React, { useState } from 'react'
import TextField from '@material-ui/core/TextField'
import Grid from '@material-ui/core/Grid'
import Typography from '@material-ui/core/Typography'
import Button from '@material-ui/core/Button'
import ButtonGroup from '@material-ui/core/ButtonGroup'
import { ipcRenderer } from 'electron'

ipcRenderer.on('signin-response', (_, arg) => {
    console.log(arg)
})

const WelcomeForm = () => {
    const [username, setUsername] = useState("")
    const [password, setPassword] = useState("")

    const signin = () => {
        ipcRenderer.send('signin', {
            username,
            password
        })
    }

    return (
        <div
            style={{
                position: 'absolute',
                left: '50%',
                top: '50%',
                transform: 'translate(-50%, -50%)'
            }}>
            <Grid
                container
                spacing={3}
                direction="row"
                alignItems="center"
                justify="center">
                <Grid item xs={12}>
                    <Typography variant="h4">
                        Welcome
                    </Typography>
                </Grid>
                <Grid item xs={12}>
                    <TextField fullWidth required label="Username" onChange={e => { setUsername(e.target.value) }} />
                </Grid>
                <Grid item xs={12}>
                    <TextField fullWidth required type="password" label="Password" onChange={e => { setPassword(e.target.value) }} />
                </Grid>
                <Grid item xs={12}>
                    <ButtonGroup disableElevation style={{ height: '48px' }} fullWidth variant="contained" color="primary">
                        <Button onClick={() => signin()}>Signin</Button>
                        <Button>Signup</Button>
                    </ButtonGroup>
                </Grid>
            </Grid>
        </div >
    )
}

export default WelcomeForm