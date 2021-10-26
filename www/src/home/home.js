import React, { useState } from 'react';
import { CircularProgress } from '@material-ui/core';
import './home.scss';

function Home() {
    // const { user } = useContext(AppContext);
    const [loading, setLoading] = useState(false);
    return (
        <div className={'home ' + (loading ? 'loading' : '')}>
            {loading && <CircularProgress className="loader" />}
            <h5>Hello World!</h5>
        </div>
    );
}

export default Home;
