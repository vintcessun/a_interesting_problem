use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use rand::Rng;
use clearscreen::clear;


struct Strategy{
    first:u32,//0是对方先出，其他是我先出几
    max:u32,//每次凑够几
}

#[derive(Debug)]
struct Rule{
    range:Vec<u32>,
    target:u32,
}

fn main(){
    test();
}

fn test() {
    let first = true;
    let range = vec![1,2,3,5,6,7];
    let target = 1283;

    let strategy = best_choice(&range,target);
    let rule = Rule{range:range,target:target};
    if strategy.max==0{
        return;
    }

    //game_simulator_rand(first,rule);

    auto_game_simulator(strategy,rule);
}

fn best_choice(range:&Vec<u32>,target:u32)->Strategy{
    let mut add_list:Vec<Vec<u32>> = Vec::with_capacity(range.len());
    for i in range{
        let mut one:Vec<u32> = Vec::with_capacity(range.len());
        for j in range{
            one.push(i+j);
        }
        add_list.push(one);
    }

    let mut chooselist = Vec::with_capacity(range.len()*range.len());
    chooselist = add_list[0].clone();
    for i in add_list{
        let mut ret = Vec::with_capacity(chooselist.len());
        for j in chooselist{
            if i.contains(&j){
                ret.push(j);
            }
        }
        chooselist = ret;
    }

    if chooselist.len()==0{
        println!("没有必胜策略");
        return Strategy{first:0,max:0};
    }
    else{
        let sele = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("选择一个和作为最优策略")
        .default(0)
        .items(&chooselist)
        .interact()
        .unwrap();
        let max = chooselist[sele];
        if target%max==0{
            println!("对方先出，让每局凑够{}会获胜",max);
        }
        else{
            println!("我方先出，第一局出{}，对方当前局和我方下一局凑够{}会获胜",target%max,max);
        }
        return Strategy{first:target%max,max:max};
    }
}

fn auto_game_simulator(strategy:Strategy,rule:Rule){//对方采用随机策略，我方采用指定策略
    let mut rng = rand::thread_rng();
    let mut win = 0;
    let mut total = 0;

    if strategy.first==0{
        loop{
            println!("对方先出");
            let mut sum = 0;
            loop{
                let other = rule.range[rng.gen_range(0..rule.range.len())];
                println!("对方出{}",other);
                sum+=other;
                if sum>=rule.target{
                    println!("对方赢了");
                    total+=1;
                    break;
                }

                
                let my = strategy.max-other;
                sum+=my;
                if sum>=rule.target{
                    println!("我赢了");
                    win+=1;
                    total+=1;
                    break;
                }

                println!("现在总共有{}",sum);
            }
            clear().unwrap();
            println!("现在胜率{:.2}%",win/total*100);
        }
    }
    else{
        loop{
            println!("我方先出");
            let mut sum = 0;
            let mut other = strategy.max - strategy.first;
            loop{
                let my = strategy.max-other;
                sum+=my;
                if sum>=rule.target{
                    println!("我赢了");
                    win+=1;
                    total+=1;
                    break;
                }

                other = rule.range[rng.gen_range(0..rule.range.len())];
                println!("对方出{}",other);
                sum+=other;
                if sum>=rule.target{
                    println!("对方赢了");
                    total+=1;
                    break;
                }
                println!("现在总共有{}",sum);
            }
            clear().unwrap();
            println!("现在胜率{:.2}%",win/total*100);
        }
    }
}

fn game_simulator_rand(first:bool,rule:Rule){//对方采用随机策略
    let mut rng = rand::thread_rng();
    let mut win = 0;
    let mut total = 0;

    if !first{
        loop{
            println!("对方先出");
            let mut sum = 0;
            loop{
                let other = rule.range[rng.gen_range(0..rule.range.len())];
                println!("对方出{}",other);
                sum+=other;
                if sum>=rule.target{
                    println!("对方赢了");
                    total+=1;
                    break;
                }

                let sele = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("我方出")
                .default(0)
                .items(&rule.range)
                .interact()
                .unwrap();
                let my = rule.range[sele];
                sum+=my;
                if sum>=rule.target{
                    println!("我赢了");
                    win+=1;
                    total+=1;
                    break;
                }

                println!("现在总共有{}",sum);
            }
            clear().unwrap();
            println!("现在胜率{:.2}%",win/total*100);
        }
    }
    else{
        loop{
            println!("我方先出");
            let mut sum = 0;
            loop{
                let sele = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("我方出")
                .default(0)
                .items(&rule.range)
                .interact()
                .unwrap();
                let my = rule.range[sele];
                sum+=my;
                if sum>=rule.target{
                    println!("我赢了");
                    win+=1;
                    total+=1;
                    break;
                }
    
                let other = rule.range[rng.gen_range(0..rule.range.len())];
                println!("对方出{}",other);
                sum+=other;
                if sum>=rule.target{
                    println!("对方赢了");
                    total+=1;
                    break;
                }
                println!("现在总共有{}",sum);
            }
            clear().unwrap();
            println!("现在胜率{:.2}%",win/total*100);
        }
    }
}