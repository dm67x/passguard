import React, { useEffect, useState } from 'react'
import { Grid, IconButton, ListItem, ListItemSecondaryAction, ListItemText, Typography } from '@material-ui/core'
import DeleteIcon from '@material-ui/icons/Delete'
import VisibilityIcon from '@material-ui/icons/Visibility'
import { ipcRenderer } from 'electron'

const PasswordRow = (props) => {
    const [visible, setVisible] = useState(false)
    const { id, url, onVisibilityChanged } = props

    useEffect(() => { onVisibilityChanged(id, visible); setVisible(false) }, [visible])

    return (
        <ListItem>
            <ListItemText>
                <Grid container>
                    <Grid item xs={12}>
                        <Typography><b>{url}</b></Typography>
                    </Grid>
                </Grid>
            </ListItemText>
            <ListItemSecondaryAction>
                <IconButton onClick={() => setVisible(!visible)}>
                    <VisibilityIcon />
                </IconButton>
                <IconButton onClick={() => ipcRenderer.send('remove-password', { id })}>
                    <DeleteIcon />
                </IconButton>
            </ListItemSecondaryAction>
        </ListItem>
    )
}

export default PasswordRow