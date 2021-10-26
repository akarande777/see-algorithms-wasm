import React, { useEffect, useState } from 'react';
import './App.scss';
import { HashRouter } from 'react-router-dom';
import { Grid, Drawer } from '@material-ui/core';
import Header from './components/header/header';
import Sider from './components/sider/sider';
import Content from './components/content/content';
import Toast from './components/toast/toast';
import Menu from './components/menu/menu';
import { AppContext } from './common/context';

function App() {
    const [visible, setVisible] = useState(false);
    const [context, setContext] = useState(null);

    useEffect(() => {
        import('see-algorithms').then((module) => {
            setContext({
                ...module,
                Graph: new module.Graph(),
            });
        });
    }, []);

    return (
        <AppContext.Provider value={context}>
            <div className="App">
                <Toast />
                <Menu />
                <Header toggleMenu={() => setVisible(!visible)} />
                <HashRouter>
                    <Drawer
                        anchor="left"
                        open={visible}
                        onClose={() => setVisible(false)}
                        className="drawer"
                        PaperProps={{ className: 'paper' }}
                        BackdropProps={{ className: 'backdrop' }}
                    >
                        <Sider onClose={() => setVisible(false)} />
                    </Drawer>
                    <Grid container className="layout">
                        <Grid item xs="auto" className="d-none d-md-block sider">
                            <Sider onClose={() => null} />
                        </Grid>
                        <Grid item xs className="content">
                            {context && <Content visible={visible} />}
                        </Grid>
                    </Grid>
                </HashRouter>
            </div>
        </AppContext.Provider>
    );
}

export default App;
