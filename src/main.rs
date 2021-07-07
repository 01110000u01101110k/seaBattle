use yew::prelude::*;
use yew::services::console::ConsoleService;

const COUNT_CELLS: u8 = 100;
struct Ship {
    count: u8,
    space: u8
}
type NewShip = Ship;
const FIVE_SPACES_SHIP: NewShip = Ship {
    count: 1,
    space: 5
};
const FOUR_SPACES_SHIP: NewShip = Ship {
    count: 2,
    space: 4
};
const THREE_SPACES_SHIP: NewShip = Ship {
    count: 3,
    space: 3
};
 const TWO_SPACES_SHIP: NewShip = Ship {
    count: 4,
    space: 2
};

enum GameState {
    PlayerTurn,
    ChangeGameState,
    PlayerCount,
    Drag
}

struct Model {
    link: ComponentLink<Self>,
    player_turn: u8,
    player_count: u8,
    game_state: bool
}

impl Component for Model {
    type Message = GameState; // пользовательский тип Message, используется что бы применять перечисление Msg внутри имплиментации
    type Properties = (); /* пользовательский тип Properties обычно имеет значение () для корневого компонента, или можно использовать App::mount_with_props, что бы указать свойства для корневого компонента. 
    Значение Props используется если это один из наследников корневого компонента, внутри Props обычно передаются данные от одного компонента другому.
    */

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            player_turn: 1,
            player_count: 2,
            game_state: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            GameState::PlayerTurn => {
                if self.player_turn == self.player_count{
                    self.player_turn = 1;
                } else {
                    self.player_turn += 1;
                }
                true
            }
            GameState::ChangeGameState => {
                self.game_state = !self.game_state;
                true
            }
            GameState::PlayerCount => {
                self.player_count += 1;
                true
            }
            GameState::Drag => {
                ConsoleService::log("Completed update");
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //let remove_one = self.link.callback(|_| Msg::RemoveOne);
        let ship_arr: [NewShip; 4] = [FIVE_SPACES_SHIP, FOUR_SPACES_SHIP,THREE_SPACES_SHIP, TWO_SPACES_SHIP];
        html! {
            <>
                <header>
                    <nav>
                        <button class="btn">{"start"}</button>
                        <button class="btn">{"reset"}</button>
                    </nav>
                </header>
                <main>
                        /*<button class="btn" onclick=self.link.callback(|_| Msg::AddOne)>{"+1" }</button>
                        <button class="btn" onclick=remove_one>{"-1"}</button>
                        <p>{ self.value }</p>
                        <p>{"some else"}</p>*/
                        
                    <div class="shipPlace">
                    {
                        for (0..ship_arr.len()).into_iter().map(|iter| {
                            html!
                            {
                                <div draggable="true"
                                ondragstart=self.link.callback(|_| GameState::Drag) class="ship">
                                    {for (0..ship_arr[iter].space).into_iter().map(|_cell| {
                                        html! {
                                            <div class="shipCell"></div>
                                        }
                                    })}
                                    <p class="countShip">{ship_arr[iter].count.to_string()}</p>
                                </div>
                            }
                        })
                    }
                    </div>    
                    <div class="container">
                        {
                            for (0..2).into_iter().map(|id|
                            {
                                html! {
                                    <div class="playField" id={id.to_string()}>
                                    {
                                        for (0..COUNT_CELLS).into_iter().map(|cell| {
                                            html! {
                                                <div class="cell" id={cell.to_string()}>{"✕ ◉"}</div>
                                            }
                                        })
                                    }
                                </div>
                                }
                            })
                        }
                        
                    </div>    
                </main>
            </>
        }
    }

    fn rendered(&mut self, first_render: bool) { // позволяет вызывать некоторые действия после вызова view и отображения визуальной составляющей, к примеру можно сделать запросы к серверу 
        if first_render {
            println!("some test");
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}