import { Button, Grid } from '@material-ui/core'
import React, { useEffect } from 'react'
import ExitToAppIcon from '@material-ui/icons/ExitToApp'
import { ipcRenderer } from 'electron'
import { useHistory } from 'react-router-dom'
import logo from '../images/icon.png'

const Profile = () => {
    const history = useHistory()

    useEffect(() => {
        ipcRenderer.on('signout-response', (_, arg) => {
            if (arg) {
                history.replace('/')
            }
        })

        return () => {
            ipcRenderer.removeAllListeners('signout-response')
        }
    }, [])

    return (
        <Grid container align="center">
            <Grid item xs={12}>
                <img src={logo} height="60px" />
            </Grid>
            <Grid item xs={12}>
                <Button onClick={() => ipcRenderer.send('signout')}>
                    <ExitToAppIcon /> Change user
            </Button>
            </Grid>
        </Grid>
    )
}

export default Profile