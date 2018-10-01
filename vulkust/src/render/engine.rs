use super::super::core::application::Application as CoreAppTrait;
use super::super::core::event::Event;
use super::super::core::timing::Timing;
use super::super::system::os::application::Application as OsApp;
use super::camera::DefaultCamera;
use super::deferred::Deferred;
use super::gapi::GraphicApiEngine;
use super::gx3d::import as gx3d_import;
use super::model::DefaultModel;
use super::multithreaded::Engine as MultithreadedEngine;
use super::scene::{DefaultScene, Loadable as LoadableScene, Manager as SceneManager};
use std::sync::{Arc, RwLock, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Engine {
    pub myself: Option<Weak<RwLock<Engine>>>,
    pub gapi_engine: Arc<RwLock<GraphicApiEngine>>,
    pub os_app: Weak<RwLock<OsApp>>,
    pub core_app: Arc<RwLock<CoreAppTrait>>,
    pub scene_manager: Arc<RwLock<SceneManager>>,
    pub deferred: Arc<RwLock<Deferred>>,
    pub timing: Arc<RwLock<Timing>>,
    multithreaded_engine: MultithreadedEngine,
}

impl Engine {
    pub fn new(core_app: Arc<RwLock<CoreAppTrait>>, os_app: &Arc<RwLock<OsApp>>) -> Self {
        let config = &vxresult!(core_app.read()).get_config();
        let gapi_engine = GraphicApiEngine::new(os_app, &config.render);
        let scene_manager = Arc::new(RwLock::new(SceneManager::new()));
        gx3d_import(&scene_manager);
        let deferred = Arc::new(RwLock::new(Deferred::new(
            &gapi_engine,
            &*vxresult!(scene_manager.read()),
        )));
        let gapi_engine = Arc::new(RwLock::new(gapi_engine));
        let myself = None;
        let multithreaded_engine =
            MultithreadedEngine::new(gapi_engine.clone(), scene_manager.clone());
        Engine {
            myself,
            gapi_engine,
            os_app: Arc::downgrade(os_app),
            core_app,
            scene_manager,
            deferred,
            timing: Arc::new(RwLock::new(Timing::new())),
            multithreaded_engine,
        }
    }

    pub fn set_myself(&mut self, myself: Weak<RwLock<Engine>>) {
        vxresult!(self.scene_manager.write()).set_engine(myself.clone());
        self.myself = Some(myself);
    }

    pub fn update(&self) {
        // todo it must iterate over scenes
        // it require separate command buffer for each scene
        // it gonna help the multithread rendering part
        // tmporary it must change in future
        // vxresult!(self.gapi_engine.write()).start_recording();
        // vxresult!(self.scene_manager.read()).render(); // temp
        // vxresult!(self.gapi_engine.read()).start_deferred();
        // todo update deferred buffer in scene
        // vxresult!(self.scene_manager.read()).render_deferred();
        // vxresult!(self.deferred.read()).render(&mut *vxresult!(self.gapi_engine.write()));
        // vxresult!(self.gapi_engine.write()).end_recording();
        self.multithreaded_engine.render();
    }

    pub fn load_gltf_scene<S>(&self, file_name: &str, scene_name: &str) -> Arc<RwLock<S>>
    where
        S: 'static + LoadableScene,
    {
        vxresult!(self.scene_manager.write()).load_gltf::<S>(file_name, scene_name)
    }

    pub fn create_scene<S>(&self) -> Arc<RwLock<S>>
    where
        S: 'static + DefaultScene,
    {
        vxresult!(self.scene_manager.write()).create()
    }

    pub fn create_camera<C>(&self) -> Arc<RwLock<C>>
    where
        C: 'static + DefaultCamera,
    {
        vxresult!(self.scene_manager.read()).create_camera()
    }

    pub fn create_model<M>(&self) -> Arc<RwLock<M>>
    where
        M: 'static + DefaultModel,
    {
        let sm = vxresult!(self.scene_manager.read());
        let mut mm = vxresult!(sm.model_manager.write());
        let m = mm.create(self);
        return m;
    }

    pub fn on_event(&self, _e: Event) {}
}

unsafe impl Send for Engine {}

unsafe impl Sync for Engine {}
