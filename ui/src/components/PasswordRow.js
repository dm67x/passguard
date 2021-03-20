import React, { useEffect, useState } from 'react'
import { Grid, IconButton, ListItem, ListItemSecondaryAction, ListItemText, Typography } from '@material-ui/core'
import DeleteIcon from '@material-ui/icons/Delete'
import VisibilityIcon from '@material-ui/icons/Visibility'
import VisibilityOffIcon from '@material-ui/icons/VisibilityOff'
import { ipcRenderer } from 'electron'

const PasswordRow = (props) => {
    const [visible, setVisible] = useState(false)
    const [password, setPassword] = useState('***********')

    useEffect(() => {
        if (visible) {
            ipcRenderer.send('decrypt-password', {
                id: props.id,
            })
        } else {
            setPassword('***********')
        }
    }, [visible])

    return (
        <ListItem>
            <ListItemText>
                <Grid container>
                    <Grid item xs={12} md={6}>
                        <Typography>{props.url}</Typography>
                    </Grid>
                    <Grid item xs={12} md={6}>
                        <Typography>{password}</Typography>
                    </Grid>
                </Grid>
            </ListItemText>
            <ListItemSecondaryAction>
                <IconButton onClick={() => setVisible(!visible)}>
                    {visible ? <VisibilityOffIcon /> : <VisibilityIcon />}
                </IconButton>
                <IconButton>
                    <DeleteIcon />
                </IconButton>
            </ListItemSecondaryAction>
        </ListItem>
    )
}

export default PasswordRow