import React, { useContext } from 'react';
import { List, ListItem, ListItemIcon, ListItemText } from '@material-ui/core';
import { showMenu } from '../menu/menu';
import { DonutLarge, Equalizer, Share } from '@material-ui/icons';
import { withRouter } from 'react-router-dom';
import './sider.scss';
import { Algorithms, DataStructures } from '../../common/constants';

function Sider({ history, ...props }) {
    // const { user } = useContext(AppContext);

    const handleSelect = (algo) => {
        history.push(algo.value);
        props.onClose();
    };

    const getMenuOptions = (e) => ({
        anchorEl: e.currentTarget,
        anchorOrigin: { vertical: 'center', horizontal: 'center' },
        onSelect: handleSelect,
    });

    return (
        <List className="list">
            <ListItem
                button
                className="listItem"
                onClick={(e) => {
                    showMenu({
                        ...getMenuOptions(e),
                        menuItems: Algorithms.sorting,
                    });
                }}
            >
                <ListItemIcon>
                    <Equalizer className="listItemIcon" />
                </ListItemIcon>
                <ListItemText primary="Sorting" className="listItemText" />
            </ListItem>
            <ListItem
                button
                className="listItem"
                onClick={(e) => {
                    showMenu({
                        ...getMenuOptions(e),
                        menuItems: Algorithms.graph,
                    });
                }}
            >
                <ListItemIcon>
                    <Share className="listItemIcon" />
                </ListItemIcon>
                <ListItemText primary="Graph" className="listItemText" />
            </ListItem>
            <ListItem
                button
                className="listItem"
                onClick={(e) => {
                    showMenu({
                        ...getMenuOptions(e),
                        menuItems: DataStructures,
                    });
                }}
            >
                <ListItemIcon>
                    <DonutLarge className="listItemIcon" />
                </ListItemIcon>
                <ListItemText primary="Data Structures" className="listItemText" />
            </ListItem>
        </List>
    );
}

export default withRouter(Sider);
